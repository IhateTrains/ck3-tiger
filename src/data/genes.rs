use crate::block::validator::Validator;
use crate::block::{Block, BlockOrValue};
use crate::errorkey::ErrorKey;
use crate::errors::{error, warn};
use crate::everything::{Db, DbKind, Everything};
use crate::item::Item;
use crate::token::Token;

#[derive(Clone, Debug)]
pub struct Gene {}

impl Gene {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        match key.as_str() {
            "color_genes" => {
                for (k, b) in block.iter_pure_definitions_warn() {
                    ColorGene::add(db, k.clone(), b.clone());
                }
            }
            "age_presets" => {
                for (k, b) in block.iter_pure_definitions_warn() {
                    AgePresetGene::add(db, k.clone(), b.clone());
                }
            }
            "decal_atlases" => {
                for (_k, _b) in block.iter_pure_definitions_warn() {
                    // TODO: no examples in vanilla
                }
            }
            "morph_genes" => {
                for (k, b) in block.iter_pure_definitions_warn() {
                    MorphGene::add(db, k.clone(), b.clone());
                }
            }
            "accessory_genes" => {
                for (k, b) in block.iter_pure_definitions_warn() {
                    AccessoryGene::add(db, k.clone(), b.clone());
                }
            }
            "special_genes" => {
                for (k, b) in block.iter_pure_definitions_warn() {
                    match k.as_str() {
                        "morph_genes" => {
                            for (k, b) in b.iter_pure_definitions_warn() {
                                MorphGene::add(db, k.clone(), b.clone());
                            }
                        }
                        "accessory_genes" => {
                            for (k, b) in b.iter_pure_definitions_warn() {
                                AccessoryGene::add(db, k.clone(), b.clone());
                            }
                        }
                        _ => warn(k, ErrorKey::ParseError, "unknown gene type"),
                    }
                }
            }
            _ => warn(key, ErrorKey::ParseError, "unknown gene type"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ColorGene {}

impl ColorGene {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::GeneCategory, key, block, Box::new(Self {}));
    }
}

impl DbKind for ColorGene {
    fn validate(&self, _key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);
        vd.req_field("group");
        vd.req_field("color");
        vd.req_field("blend_range");

        vd.field_item("sync_inheritance_with", Item::GeneCategory);
        vd.field_value("group"); // TODO
        vd.field_value("color"); // TODO
        vd.field_validated_block("blend_range", validate_gene_range);
    }
}

#[derive(Clone, Debug)]
pub struct AgePresetGene {}

impl AgePresetGene {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::GeneAgePreset, key, block, Box::new(Self {}));
    }
}

impl DbKind for AgePresetGene {
    fn validate(&self, _key: &Token, block: &Block, data: &Everything) {
        validate_age(block, data);
    }
}

#[derive(Clone, Debug)]
pub struct MorphGene {}

impl MorphGene {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::GeneCategory, key, block, Box::new(Self {}));
    }
}

impl DbKind for MorphGene {
    fn validate(&self, _key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);

        vd.field_list("ugliness_feature_categories"); // TODO: options
        vd.field_bool("can_have_portrait_extremity_shift");
        vd.field_value("group"); // TODO
        for (_key, bv) in vd.unknown_keys() {
            if let Some(block) = bv.expect_block() {
                validate_morph_gene(block, data);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct AccessoryGene {}

impl AccessoryGene {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::GeneCategory, key, block, Box::new(Self {}));
    }
}

impl DbKind for AccessoryGene {
    fn validate(&self, _key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);

        vd.field_bool("inheritable");
        vd.field_value("group"); // TODO, is it even used? only one example in vanilla
        for (_key, bv) in vd.unknown_keys() {
            if let Some(block) = bv.expect_block() {
                validate_accessory_gene(block, data);
            }
        }
    }
}

fn validate_age_field(bv: &BlockOrValue, data: &Everything) {
    match bv {
        BlockOrValue::Value(token) => data.verify_exists(Item::GeneAgePreset, token),
        BlockOrValue::Block(block) => validate_age(block, data),
    }
}

fn validate_age(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    vd.req_field("mode");
    vd.req_field("curve");

    vd.field_value("mode"); // TODO
    vd.field_validated_block("curve", validate_curve);
}

fn validate_curve(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    for block in vd.blocks() {
        validate_curve_range(block, data);
    }
}

fn validate_hsv_curve(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    for block in vd.blocks() {
        validate_hsv_curve_range(block, data);
    }
}

fn validate_curve_range(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    let mut count = 0;
    for token in vd.values() {
        if let Ok(v) = token.as_str().parse::<f64>() {
            count += 1;
            if count == 1 {
                if v < 0.0 || v > 1.0 {
                    error(token, ErrorKey::Range, "expected number from 0.0 to 1.0");
                }
            } else {
                if v < -1.0 || v > 1.0 {
                    error(token, ErrorKey::Range, "expected number from -1.0 to 1.0");
                }
            }
        } else {
            error(token, ErrorKey::Validation, "expected number");
        }
    }
    if count != 2 {
        error(block, ErrorKey::Validation, "expected exactly 2 numbers");
    }
}

fn validate_hsv_curve_range(block: &Block, data: &Everything) {
    let mut found_first = false;
    let mut found_second = false;

    for (k, _cmp, bv) in block.iter_items() {
        if let Some(key) = k {
            warn(key, ErrorKey::Validation, "unknown field");
        } else if !found_first {
            if let Some(token) = bv.expect_value() {
                if let Ok(v) = token.as_str().parse::<f64>() {
                    found_first = true;
                    if v < 0.0 || v > 1.0 {
                        error(token, ErrorKey::Range, "expected number from 0.0 to 1.0");
                    }
                } else {
                    error(token, ErrorKey::Validation, "expected number");
                }
            }
        } else if !found_second {
            if let Some(block) = bv.expect_block() {
                found_second = true;
                let mut count = 0;
                let mut vd = Validator::new(block, data);
                for token in vd.values() {
                    if let Ok(v) = token.as_str().parse::<f64>() {
                        count += 1;
                        if v < -1.0 || v > 1.0 {
                            error(token, ErrorKey::Range, "expected number from -1.0 to 1.0");
                        }
                    } else {
                        error(token, ErrorKey::Validation, "expected number");
                    }
                }
                if count != 3 {
                    error(block, ErrorKey::Validation, "expected exactly 3 numbers");
                }
            }
        }
    }
}

fn validate_gene_range(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    let mut count = 0;
    for token in vd.values() {
        if let Ok(v) = token.as_str().parse::<f64>() {
            count += 1;
            if v < 0.0 || v > 1.0 {
                error(token, ErrorKey::Range, "expected number from 0.0 to 1.0");
            }
        } else {
            error(token, ErrorKey::Validation, "expected number");
        }
    }
    if count != 2 {
        error(block, ErrorKey::Validation, "expected exactly 2 numbers");
    }
}

fn validate_morph_gene(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    vd.req_field("index");
    vd.field_integer("index"); // TODO: verify unique indices
    vd.field_bool("generic");
    vd.field_bool("visible");
    vd.field_value("positive_mirror"); // TODO
    vd.field_value("negative_mirror"); // TODO
    let choices = &["male", "female", "boy", "girl"];
    for field in choices {
        vd.field_validated(field, |bv, data| {
            match bv {
                BlockOrValue::Value(token) => {
                    // TODO: if it refers to another field, check that following the chain of fields eventually reaches a block
                    if !choices.contains(&token.as_str()) {
                        let msg = format!("expected one of {}", choices.join(", "));
                        warn(token, ErrorKey::Validation, &msg);
                    }
                }
                BlockOrValue::Block(block) => {
                    let mut vd = Validator::new(block, data);
                    vd.field_validated_blocks("setting", validate_gene_setting);
                    vd.field_validated_blocks("decal", validate_gene_decal);
                    vd.field_validated_blocks("texture_override", validate_texture_override);
                    vd.field_validated_block("hair_hsv_shift_curve", validate_shift_curve);
                    vd.field_validated_block("eye_hsv_shift_curve", validate_shift_curve);
                    vd.field_validated_block("skin_hsv_shift_curve", validate_shift_curve);
                }
            }
        });
    }
}

fn validate_accessory_gene(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    vd.req_field("index");
    vd.field_integer("index"); // TODO: verify unique indices
    let choices = &["male", "female", "boy", "girl"];
    for field in choices {
        vd.field_validated(field, |bv, data| {
            match bv {
                BlockOrValue::Value(token) => {
                    // TODO: if it refers to another field, check that following the chain of fields eventually reaches a block
                    if !choices.contains(&token.as_str()) {
                        let msg = format!("expected one of {}", choices.join(", "));
                        warn(token, ErrorKey::Validation, &msg);
                    }
                }
                BlockOrValue::Block(block) => {
                    let mut vd = Validator::new(block, data);
                    for (_weight, token) in vd.integer_values() {
                        data.verify_exists(Item::GfxPortraitsAccessories, token);
                    }
                }
            }
        });
    }
}

fn validate_gene_setting(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    vd.req_field("attribute");
    vd.req_field_one_of(&["value", "curve"]);
    vd.field_item("attribute", Item::GeneAttribute);
    vd.field_validated_block("value", |block, data| {
        let mut vd = Validator::new(block, data);
        vd.req_field("min");
        vd.req_field("max");
        vd.field_numeric("min");
        vd.field_numeric("max");
    });
    vd.field_validated_block("curve", validate_curve);
    vd.field_validated("age", validate_age_field);
    vd.field_value("required_tags"); // TODO
}

fn validate_gene_decal(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    vd.req_field("body_part");
    vd.req_field("textures");
    vd.req_field("priority");
    vd.field_value("body_part"); // TODO
    vd.field_validated_blocks("textures", validate_decal_textures);
    vd.field_validated_blocks("alpha_curve", validate_curve);
    vd.field_validated_blocks("blend_modes", validate_blend_modes);
    vd.field_integer("priority");
    vd.field_validated("age", validate_age_field);
    vd.field_choice("decal_apply_order", &["pre_skin_color", "post_skin_color"]);
}

fn validate_decal_textures(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    // TODO: validate that it's a dds? What properties should the dds have?
    vd.field_item("diffuse", Item::File);
    vd.field_item("normal", Item::File);
    vd.field_item("specular", Item::File);
    vd.field_item("properties", Item::File);
}

fn validate_texture_override(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    vd.req_field("weight");
    vd.field_integer("weight");
    // TODO: validate that it's a dds? What properties should the dds have?
    vd.field_item("diffuse", Item::File);
    vd.field_item("normal", Item::File);
    vd.field_item("specular", Item::File);
    vd.field_item("properties", Item::File);
}

fn validate_blend_modes(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    let choices = &["overlay", "replace", "hard_light", "multiply"];
    vd.field_choice("diffuse", choices);
    vd.field_choice("normal", choices);
    vd.field_choice("properties", choices);
}

fn validate_shift_curve(block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);
    vd.req_field("curve");
    vd.field_validated_block("curve", validate_hsv_curve);
    vd.field_validated("age", validate_age_field);
}