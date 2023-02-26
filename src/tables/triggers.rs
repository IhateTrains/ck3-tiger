use crate::everything::Everything;
use crate::item::Item;
use crate::scopes::*;
use crate::token::Token;

pub fn scope_trigger_target(name: &Token, data: &Everything) -> Option<(Scopes, Scopes)> {
    for (from, s, to) in SCOPE_TRIGGER_TARGET {
        if name.is(s) {
            return Some((
                Scopes::from_bits_truncate(*from),
                Scopes::from_bits_truncate(*to),
            ));
        }
    }
    if let Some(relation) = name.as_str().strip_prefix("has_relation_") {
        if data.relations.exists(relation) {
            return Some((Scopes::Character, Scopes::Character));
        }
    }
    if let Some(relation) = name.as_str().strip_prefix("has_secret_relation_") {
        if data.relations.exists(relation) {
            return Some((Scopes::Character, Scopes::Character));
        }
    }
    std::option::Option::None
}

pub fn scope_trigger_bool(name: &str) -> Option<Scopes> {
    for (from, s) in SCOPE_TRIGGER_BOOL {
        if *s == name {
            return Some(Scopes::from_bits_truncate(*from));
        }
    }
    std::option::Option::None
}

pub fn scope_trigger_item(name: &str) -> Option<(Scopes, Item)> {
    for (from, s, item) in SCOPE_TRIGGER_ITEM {
        if *s == name {
            return Some((Scopes::from_bits_truncate(*from), *item));
        }
    }
    std::option::Option::None
}

/// LAST UPDATED VERSION 1.8.1
/// See `triggers.log` from the game data dumps
/// These are the triggers that do a simple comparison with a target scope item
const SCOPE_TRIGGER_TARGET: &[(u32, &str, u32)] = &[
    (Character, "can_attack_in_hierarchy", Character),
    (Character, "can_be_child_of", Character),
    (Artifact, "can_be_claimed_by", Character),
    (Secret, "can_be_exposed_by", Character),
    (Character, "can_be_parent_of", Character),
    (Character, "can_benefit_from_artifact", Artifact),
    (Character, "can_equip_artifact", Artifact),
    (Culture, "can_get_innovation_from", Culture),
    (Character, "can_hybridize", Culture),
    (Character, "can_hybridize_excluding_cost", Culture),
    (Character, "can_join_faction", Faction),
    (Character, "can_join_or_create_faction_against", Character),
    (Character, "can_sponsor_inspiration", Inspiration),
    (
        Character,
        "character_has_commander_trait_scope_does_not",
        Character,
    ),
    (Character, "character_is_land_realm_neighbor", Character),
    (Character, "character_is_realm_neighbor", Character),
    (Character, "completely_controls", LandedTitle),
    (LandedTitle, "de_jure_drifting_towards", LandedTitle),
    (Faith, "has_allowed_gender_for_clergy", Character),
    (Character, "has_any_cb_on", Character),
    (Character, "has_any_display_cb_on", Character),
    (Character, "has_any_scripted_relation", Character),
    (Character, "has_any_secret_relation", Character),
    (Character, "has_artifact_claim", Artifact),
    (Character, "has_banish_reason", Character),
    (LandedTitle, "has_character_nominiated", Character), // sic
    (Character, "has_claim_on", LandedTitle),
    (Character, "has_court_language_of_culture", Culture),
    (Character, "has_culture", Culture),
    (Character, "has_de_jure_claim_on", Character),
    (Character, "has_disable_non_aggression_pacts", Character), // sic
    (Character, "has_divorce_reason", Character),
    (Faith, "has_dominant_ruling_gender", Character),
    (Character, "has_execute_reason", Character),
    (Character, "has_faith", Faith),
    (GreatHolyWar, "has_forced_defender", Character),
    (Character, "has_hook", Character),
    (Character, "has_hook_from_secret", Character),
    (DynastyHouse, "has_house_artifact_claim", Artifact),
    (Character, "has_imprisonment_reason", Character),
    (CharacterMemory, "has_memory_participant", Character),
    (Character, "has_non_aggression_pact", Character),
    (Character, "has_non_interference", Character),
    (Character, "has_personal_artifact_claim", Artifact),
    (GreatHolyWar, "has_pledged_attacker", Character),
    (GreatHolyWar, "has_pledged_defender", Character),
    (Faith, "has_preferred_gender_for_clergy", Character),
    (Character, "has_primary_title", LandedTitle),
    (Character, "has_raid_immunity_against", Character),
    // Special: has_relation_<relation> Character to Character
    (Character, "has_relation_to", Character),
    (Character, "has_religion", Religion),
    (Character, "has_revoke_title_reason", Character),
    (Character, "has_same_court_language", Character),
    (Character, "has_same_court_type_as", Character),
    (Character, "has_same_culture_as", Character),
    (Culture, "has_same_culture_ethos", Culture),
    (Culture, "has_same_culture_heritage", Culture),
    (Culture, "has_same_culture_language", Culture),
    (Culture, "has_same_culture_martial_tradition", Culture),
    (Character, "has_same_focus_as", Character),
    (Character, "has_same_government", Character),
    (Character, "has_same_sinful_trait", Character),
    (Character, "has_same_virtue_trait", Character),
    // Special: has_secret_relation_<relation> Character to Character
    (Character, "has_strong_claim_on", LandedTitle),
    (Character, "has_strong_hook", Character),
    (Character, "has_strong_usable_hook", Character),
    (Character, "has_title", LandedTitle),
    (Character, "has_truce", Character),
    (Character, "has_usable_hook", Character),
    (Character, "has_weak_claim_on", LandedTitle),
    (Character, "has_weak_hook", Character),
    (Character, "in_activity_with", Character),
    (Character, "in_diplomatic_range", Character),
    (Character, "is_agent_exposed_in_scheme", Scheme),
    (Character, "is_allied_in_war", Character),
    (Character, "is_allied_to", Character),
    (Army, "is_army_in_siege_relevant_for", Character),
    (Character, "is_at_location", Province),
    (Character, "is_at_same_location", Character),
    (Character, "is_at_war_with", Character),
    (War, "is_attacker", Character),
    (Character, "is_attacker_in_war", War),
    (Character, "is_attracted_to_gender_of", Character),
    (Character, "is_causing_raid_hostility_towards", Character),
    (Character, "is_child_of", Character),
    (Character, "is_close_family_of", Character),
    (Character, "is_close_or_extended_family_of", Character),
    (Character, "is_concubine_of", Character),
    (Character, "is_consort_of", Character),
    (Character, "is_councillor_of", Character),
    (Character, "is_courtier_of", Character),
    (Character, "is_cousin_of", Character),
    (Secret, "is_criminal_for", Character),
    (Struggle, "is_culture_involved_in_struggle", Culture),
    (
        LandedTitle,
        "is_de_facto_liege_or_above_target",
        LandedTitle,
    ),
    (LandedTitle, "is_de_jure_liege_or_above_target", LandedTitle),
    (War, "is_defender", Character),
    (Character, "is_defender_in_war", War),
    (Character, "is_employer_of", Character),
    (Character, "is_extended_family_of", Character),
    (Struggle, "is_faith_involved_in_struggle", Faith),
    (Character, "is_forbidden_from_scheme", Scheme),
    (Character, "is_forced_into_scheme", Scheme),
    (Character, "is_foreign_court_guest_of", Character),
    (Character, "is_foreign_court_or_pool_guest_of", Character),
    (Character, "is_grandchild_of", Character),
    (Character, "is_grandparent_of", Character),
    (Character, "is_great_grandchild_of", Character),
    (Character, "is_great_grandparent_of", Character),
    (Character, "is_heir_of", Character),
    (LandedTitle, "is_holy_site_controlled_by", Character),
    (LandedTitle, "is_holy_site_of", Faith),
    (Character, "is_imprisoned_by", Character),
    (Character, "is_in_pool_at", Province),
    (Character, "is_in_target_activity", Activity),
    (Character, "is_in_the_same_court_as", Character),
    (Character, "is_in_the_same_court_as_or_guest", Character),
    (Character, "is_knight_of", Character),
    (Secret, "is_known_by", Character),
    (Character, "is_leader_in_war", War),
    (Character, "is_liege_or_above_of", Character),
    (LandedTitle, "is_neighbor_to_realm", Character),
    (Character, "is_nibling_of", Character),
    (Character, "is_obedient", Character),
    (Character, "is_parent_of", Character),
    (War, "is_participant", Character),
    (Character, "is_participant_in_war", War),
    (Character, "is_player_heir_of", Character),
    (Character, "is_pool_guest_of", Character),
    (Character, "is_powerful_vassal_of", Character),
    (Character, "is_primary_heir_of", Character),
    (Scheme, "is_scheme_agent_exposed", Character),
    (Secret, "is_shunned_for", Character),
    (Secret, "is_shunned_or_criminal_for", Character),
    (Character, "is_sibling_of", Character),
    (Secret, "is_spent_by", Character),
    (Character, "is_spouse_of", Character),
    (Character, "is_spouse_of_even_if_dead", Character),
    (Activity, "is_target_participating", Character),
    (Character, "is_twin_of", Character),
    (Character, "is_uncle_or_aunt_of", Character),
    (Character, "is_valid_as_agent_in_scheme", Scheme),
    (Character, "is_vassal_of", Character),
    (Character, "is_vassal_or_below_of", Character),
    (War, "is_war_leader", Character),
    (Character, "knows_court_language_of", Character),
    (Character, "knows_language_of_culture", Culture),
    (Secret, "same_secret_type_as", Secret),
    (Scheme, "scheme_is_character_agent", Character),
    (Character, "sex_opposite_of", Character),
    (Character, "sex_same_as", Character),
    (LandedTitle, "can_title_join_faction", Faction),
    (
        LandedTitle,
        "target_is_de_facto_liege_or_above",
        LandedTitle,
    ),
    (LandedTitle, "target_is_de_jure_liege_or_above", LandedTitle),
    (Character, "target_is_liege_or_above", Character),
    (Character, "target_is_same_character_or_above", Character),
    (Character, "target_is_vassal_or_below", Character),
    (
        LandedTitle,
        "title_will_leave_sub_realm_on_succession",
        Character,
    ),
    (War, "was_called", Character),
];

/// LAST UPDATED VERSION 1.8.1
/// See `triggers.log` from the game data dumps
/// These are the triggers that take a simple yes or no
const SCOPE_TRIGGER_BOOL: &[(u32, &str)] = &[
    (Activity, "activity_has_been_activated"),
    (Character, "allowed_concubines"),
    (Character, "allowed_more_concubines"),
    (Character, "allowed_more_spouses"),
    (None, "always"),
    (Army, "army_is_moving"),
    (LandedTitle, "can_be_leased_out"),
    (Army, "can_disband_army"),
    (Character, "can_diverge"),
    (Character, "can_diverge_excluding_cost"),
    (CouncilTask, "can_fire_position"),
    (Character, "can_have_children"),
    (Character, "can_join_activities"),
    (None, "debug_only"),
    (
        Character,
        "does_ai_liege_in_vassal_contract_desire_obligation_change",
    ),
    (
        Character,
        "does_ai_vassal_in_vassal_contract_desire_obligation_change",
    ),
    (Dynasty, "dynasty_can_unlock_relevant_perk"),
    (Faction, "faction_can_press_demands"),
    (Faction, "faction_is_at_war"),
    (Character, "has_any_artifact"),
    (Character, "has_any_artifact_claim"),
    (Character, "has_any_court_position"),
    (Character, "has_any_focus"),
    (Character, "has_any_nickname"),
    (Character, "has_any_secrets"),
    (Character, "has_any_unequipped_artifact"),
    (Character, "has_bad_nickname"),
    (Character, "has_completed_inspiration"),
    (LandedTitle, "has_disabled_building"),
    (Character, "has_dynasty"),
    (Character, "has_employed_any_court_position"),
    (Character, "has_father"),
    (Province, "has_free_building_slot"),
    (Character, "has_free_council_slot"),
    (Province, "has_holding"),
    (None, "has_local_player_open_court_event"),
    (None, "has_local_player_seen_unopened_court_event"),
    (None, "has_local_player_unopened_court_event"),
    (Character, "has_mother"),
    (None, "has_multiple_players"),
    (Province, "has_ongoing_construction"),
    (Character, "has_outstanding_artifact_claims"),
    (Character, "has_owned_scheme"),
    (Character, "has_pending_court_events"),
    (Character, "has_prisoners"),
    (Character, "has_raised_armies"),
    (LandedTitle, "has_revokable_lease"),
    (Character, "has_royal_court"),
    (Character, "has_spawned_court_events"),
    (Province, "has_special_building"),
    (Province, "has_special_building_slot"),
    (Faction, "has_special_character"),
    (Faction, "has_special_title"),
    (Character, "has_targeting_faction"),
    (LandedTitle, "has_user_set_coa"),
    (War, "has_valid_casus_belli"),
    (LandedTitle, "has_wrong_holding_type"),
    (Character, "holds_landed_title"),
    (Character, "is_a_faction_leader"),
    (Character, "is_a_faction_member"),
    (Character, "is_adult"),
    (Character, "is_ai"),
    (Character, "is_alive"),
    (Army, "is_army_in_combat"),
    (Army, "is_army_in_raid"),
    (Army, "is_army_in_siege"),
    (Character, "is_at_home"),
    (Character, "is_at_war"),
    (Character, "is_at_war_as_attacker"),
    (Character, "is_at_war_as_defender"),
    (Character, "is_at_war_with_liege"),
    (Character, "is_attracted_to_men"),
    (Character, "is_attracted_to_women"),
    (Character, "is_away_from_court"),
    (Character, "is_betrothed"),
    (LandedTitle, "is_capital_barony"),
    (Character, "is_character_window_main_character"),
    (War, "is_civil_war"),
    (Character, "is_claimant"),
    (Character, "is_clergy"),
    (Province, "is_coastal"),
    (LandedTitle, "is_coastal_county"),
    (CombatSide, "is_combat_side_attacker"),
    (CombatSide, "is_combat_side_pursuing"),
    (CombatSide, "is_combat_side_retreating"),
    (Character, "is_commanding_army"),
    (Character, "is_concubine"),
    (LandedTitle, "is_contested"),
    (Character, "is_councillor"),
    (Province, "is_county_capital"),
    (Character, "is_courtier"),
    (GreatHolyWar, "is_directed_ghw"),
    (Culture, "is_divergent_culture"),
    (Artifact, "is_equipped"),
    (Character, "is_female"),
    (Character, "is_forced_into_faction"),
    (Character, "is_foreign_court_guest"),
    (Character, "is_foreign_court_or_pool_guest"),
    (Character, "is_from_ruler_designer"),
    (None, "is_gamestate_tutorial_active"),
    (LandedTitle, "is_head_of_faith"),
    (LandedTitle, "is_holy_order"),
    (LandedTitle, "is_holy_site"),
    (Scheme, "is_hostile"),
    (Culture, "is_hybrid_culture"),
    (Character, "is_immortal"),
    (Character, "is_imprisoned"),
    (Character, "is_in_an_activity"),
    (Character, "is_in_army"),
    (Character, "is_in_civil_war"),
    (Character, "is_in_ongoing_great_holy_war"),
    (Character, "is_incapable"),
    (Character, "is_independent_ruler"),
    (Character, "is_knight"),
    (Character, "is_landed"),
    (Character, "is_landless_ruler"),
    (LandedTitle, "is_landless_type_title"),
    (LandedTitle, "is_leased_out"),
    (Character, "is_local_player"),
    (Character, "is_lowborn"),
    (Character, "is_male"),
    (Character, "is_married"),
    (LandedTitle, "is_mercenary_company"),
    (Character, "is_normal_councillor"),
    (Character, "is_overriding_designated_winner"),
    (None, "is_player_selected"),
    (Character, "is_pledged_ghw_attacker"),
    (Character, "is_pool_character"),
    (Character, "is_pool_guest"),
    (Character, "is_powerful_vassal"),
    (Character, "is_pregnant"),
    (Army, "is_raid_army"),
    (Province, "is_raided"),
    (LandedTitle, "is_riverside_county"),
    (Province, "is_riverside_province"),
    (Character, "is_ruler"),
    (Scheme, "is_scheme_exposed"),
    (Province, "is_sea_province"),
    (Character, "is_theocratic_lessee"),
    (LandedTitle, "is_title_created"),
    (LandedTitle, "is_titular"),
    (None, "is_tutorial_active"),
    (Character, "is_unborn_child_of_concubine"),
    (Character, "is_unborn_known_bastard"),
    (LandedTitle, "is_under_holy_order_lease"),
    (Artifact, "is_unique"),
    (Character, "is_visibly_fertile"),
    (War, "is_white_peace_possible"),
    (Secret, "local_player_knows_this_secret"),
    (Character, "matrilinear_betrothal"),
    (Character, "matrilinear_marriage"),
    (Character, "owns_a_story"),
    (Character, "owns_an_activity"),
    (Character, "patrilinear_betrothal"),
    (Character, "patrilinear_marriage"),
    (None, "release_only"),
    (None, "scripted_tests"),
    (Artifact, "should_decay"),
    (None, "should_show_disturbing_portrait_modifiers"),
    (None, "should_show_nudity"),
    (LandedTitle, "title_is_a_faction_member"),
    (Character, "vassal_contract_has_modifiable_obligations"),
    (Character, "vassal_contract_is_blocked_from_modification"),
];

/// LAST UPDATED VERSION 1.8.1
/// See `triggers.log` from the game data dumps
/// These are the triggers that compare to an item type
const SCOPE_TRIGGER_ITEM: &[(u32, &str, Item)] = &[
    (Artifact, "artifact_slot_type", Item::ArtifactSlot),
    (Artifact, "artifact_type", Item::Artifact),
    (Character, "can_execute_decision", Item::Decision),
    (Artifact, "category", Item::ArtifactCategory),
    (Character, "completely_controls_region", Item::Region),
    (Faith, "controls_holy_site", Item::HolySite),
    (Faith, "controls_holy_site_with_flag", Item::HolySiteFlag),
    (
        Culture,
        "culture_overlaps_geographical_region",
        Item::Region,
    ),
    (Faction, "faction_is_type", Item::Faction),
    (Province, "geographical_region", Item::Region),
    (Artifact, "has_artifact_feature", Item::ArtifactFeature),
    (
        Artifact,
        "has_artifact_feature_group",
        Item::ArtifactFeatureGroup,
    ),
    (Artifact, "has_artifact_modifier", Item::ArtifactModifier),
    (Province, "has_building", Item::Building),
    (Culture, "has_building_gfx", Item::BuildingGfx),
    (Province, "has_building_or_higher", Item::Building),
    (Character, "has_character_modifier", Item::Modifier),
    (
        Character,
        "has_character_modifier_duration_remaining",
        Item::Modifier,
    ),
    (Culture, "has_clothing_gfx", Item::ClothingGfx),
    (Culture, "has_coa_gfx", Item::CoaGfx),
    (Province, "has_construction_with_flag", Item::BuildingFlag),
    (LandedTitle, "has_county_modifier", Item::Modifier),
    (
        LandedTitle,
        "has_county_modifier_duration_remaining",
        Item::Modifier,
    ),
    (Culture, "has_cultural_era_or_later", Item::CultureEra),
    (Culture, "has_cultural_parameter", Item::CultureParameter),
    (Culture, "has_cultural_pillar", Item::CulturePillar),
    (Culture, "has_cultural_tradition", Item::CultureTradition),
    (Faith, "has_doctrine", Item::Doctrine),
    (Faith, "has_doctrine_parameter", Item::DoctrineParameter),
    (Dynasty, "has_dynasty_modifier", Item::Modifier),
    (
        Dynasty,
        "has_dynasty_modifier_duration_remaining",
        Item::Modifier,
    ),
    (Dynasty, "has_dynasty_perk", Item::DynastyPerk),
    (Faith, "has_graphical_faith", Item::GraphicalFaith),
    (Province, "has_holding_type", Item::Holding),
    (LandedTitle, "has_holy_site_flag", Item::HolySiteFlag),
    (DynastyHouse, "has_house_modifier", Item::Modifier),
    (
        DynastyHouse,
        "has_house_modifier_duration_remaining",
        Item::Modifier,
    ),
    (Faith, "has_icon", Item::FaithIcon),
    (Character, "has_inactive_trait", Item::Trait),
    (Culture, "has_innovation", Item::Innovation),
    (Culture, "has_innovation_flag", Item::InnovationFlag),
    (Inspiration, "has_inspiration_type", Item::Inspiration),
    (Character, "has_lifestyle", Item::Lifestyle),
    (CombatSide, "has_maa_of_type", Item::MenAtArms),
    (Culture, "has_name_list", Item::NameList),
    (
        Character,
        "has_pending_interaction_of_type",
        Item::Interaction,
    ),
    (Character, "has_opposite_relation", Item::Relation),
    (Culture, "has_primary_name_list", Item::NameList),
    (Province, "has_province_modifier", Item::Modifier),
    (
        Province,
        "has_province_modifier_duration_remaining",
        Item::Modifier,
    ),
    (None, "has_reward_item", Item::RewardItem),
    (Scheme, "has_scheme_modifier", Item::Modifier),
    (
        Struggle,
        "has_struggle_phase_parameter",
        Item::StrugglePhaseParameter,
    ),
    (Character, "has_trait", Item::Trait),
    (LandedTitle, "has_title_law", Item::TitleLaw),
    (LandedTitle, "has_title_law_flag", Item::TitleLawFlag),
    (Culture, "has_unit_gfx", Item::UnitGfx),
    (Character, "is_decision_on_cooldown", Item::Decision),
    (Character, "is_leading_faction_type", Item::Faction),
    (Struggle, "is_struggle_phase", Item::StrugglePhase),
    (Struggle, "is_struggle_type", Item::Struggle),
    (LandedTitle, "is_target_of_council_task", Item::CouncilTask),
    (Character, "is_valid_for_event_debug", Item::Event), // will not work in release mode
    (Character, "is_valid_for_event_debug_cooldown", Item::Event), // will not work in release mode
    (Character, "owns_story_of_type", Item::Story),
    (Struggle, "phase_has_catalyst", Item::Catalyst),
    (Artifact, "rarity", Item::ArtifactRarity),
    (Faith, "religion_tag", Item::Religion),
    (Scheme, "scheme_skill", Item::Skill),
    (Scheme, "scheme_type", Item::Scheme),
    (Secret, "secret_type", Item::Secret),
    (StoryCycle, "story_type", Item::Story),
    (Province, "terrain", Item::Terrain),
    (Faith, "trait_is_sin", Item::Trait),
    (Faith, "trait_is_virtue", Item::Trait),
    (War, "using_cb", Item::CasusBelli),
];
