use crate::block::validator::Validator;
use crate::block::BV;
use crate::context::ScopeContext;
use crate::everything::Everything;
use crate::item::Item;
use crate::tooltipped::Tooltipped;
use crate::trigger::validate_normal_trigger;

pub fn validate_scripted_animation(bv: &BV, data: &Everything, sc: &mut ScopeContext) {
    match bv {
        BV::Value(token) => data.verify_exists(Item::ScriptedAnimation, token),
        BV::Block(block) => {
            let mut vd = Validator::new(block, data);
            vd.field_validated_blocks("triggered_animation", |block, data| {
                let mut vd = Validator::new(block, data);
                vd.field_validated_block("trigger", |block, data| {
                    validate_normal_trigger(block, data, sc, Tooltipped::No);
                });
                vd.field_validated("animation", validate_animation);
            });
            vd.field_validated("animation", validate_animation);
            vd.field_item("scripted_animation", Item::ScriptedAnimation);
        }
    }
}

fn validate_animation(bv: &BV, data: &Everything) {
    match bv {
        BV::Value(token) => data.verify_exists(Item::PortraitAnimation, token),
        BV::Block(block) => {
            let mut vd = Validator::new(block, data);
            for token in vd.values() {
                data.verify_exists(Item::PortraitAnimation, token);
            }
        }
    }
}