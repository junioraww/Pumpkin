use std::sync::atomic::Ordering;

use pumpkin_data::Block;
use pumpkin_data::tag::{self, Taggable};

use super::{Controls, Goal};
use crate::entity::mob::Mob;

pub struct ClimbOnTopOfPowderSnowGoal {
    goal_control: Controls,
}

impl Default for ClimbOnTopOfPowderSnowGoal {
    fn default() -> Self {
        Self {
            goal_control: Controls::JUMP,
        }
    }
}

impl ClimbOnTopOfPowderSnowGoal {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            goal_control: Controls::JUMP,
        }
    }
}

impl Goal for ClimbOnTopOfPowderSnowGoal {
    fn can_start(&mut self, mob: &dyn Mob) -> bool {
        let entity = mob.get_entity();
        let in_powder_snow = entity.was_in_powder_snow.load(Ordering::Relaxed)
            || entity.is_in_powder_snow.load(Ordering::Relaxed);

        if in_powder_snow
            && entity
                .entity_type
                .has_tag(&tag::EntityType::MINECRAFT_POWDER_SNOW_WALKABLE_MOBS)
        {
            let world = entity.world.load();
            let above = entity.block_pos.load().up();
            let above_state = world.get_block_state(&above);
            let above_block = above_state.id.to_block();
            above_block.id == Block::POWDER_SNOW.id || above_state.collision_shapes.is_empty()
        } else {
            false
        }
    }

    fn should_continue(&mut self, mob: &dyn Mob) -> bool {
        self.can_start(mob)
    }

    fn tick(&mut self, mob: &dyn Mob) {
        mob.get_mob_entity()
            .jump_control
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .jump();
    }

    fn should_run_every_tick(&self) -> bool {
        true
    }

    fn controls(&self) -> Controls {
        self.goal_control
    }
}
