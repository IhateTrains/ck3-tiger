use crate::everything::Everything;
use crate::item::Item;
use crate::scopes::*;
use crate::token::Token;
use crate::trigger::Trigger;

use Trigger::*;

pub fn scope_trigger(name: &Token, _data: &Everything) -> Option<(Scopes, Trigger)> {
    let name_lc = name.as_str().to_lowercase();

    for (from, s, trigger) in TRIGGER {
        if name_lc == *s {
            return Some((*from, *trigger));
        }
    }
    std::option::Option::None
}

/// LAST UPDATED VIC3 VERSION 1.3.6
/// See `triggers.log` from the game data dumps
/// A key ends with '(' if it is the version that takes a parenthesized argument in script.
const TRIGGER: &[(Scopes, &str, Trigger)] = &[
    (Scopes::None, "active_lens", UncheckedValue), // no examples in vanilla
    (Scopes::None, "active_lens_option", UncheckedValue), // no examples in vanilla
    // TODO: warn if this is in an any_ iterator and not at the end
    (Scopes::all_but_none(), "add_to_temporary_list", Special),
    (Scopes::Character, "age", CompareValue),
    (Scopes::Country, "aggressive_diplomatic_plays_permitted", Boolean),
    (Scopes::Country, "ai_army_comparison(", CompareValue),
    (Scopes::Country, "ai_gdp_comparison(", CompareValue),
    (Scopes::Country, "ai_ideological_opinion(", CompareValue),
    (Scopes::Country, "ai_navy_comparison(", CompareValue),
    (Scopes::None, "all_false", Control),
    (Scopes::None, "always", Boolean),
    (Scopes::None, "and", Control),
    (Scopes::None, "any_false", Control),
    (Scopes::Country, "approaching_bureaucracy_shortage", Boolean),
    (Scopes::State, "arable_land", CompareValue),
    (Scopes::Country, "arable_land_country", CompareValue),
    (Scopes::Country, "army_reserves", CompareValue),
    (Scopes::None, "assert_if", Block(&[("limit", Control), ("?text", UncheckedValue)])),
    (Scopes::None, "assert_read", UncheckedValue),
    (Scopes::Country, "authority", CompareValue),
    (Scopes::State, "available_jobs", CompareValue),
    (Scopes::Country, "average_country_infrastructure", CompareValue),
    (Scopes::Front, "average_defense(", CompareValue),
    (Scopes::Country, "average_incorporated_country_infrastructure", CompareValue),
    (Scopes::Front, "average_offense(", CompareValue),
    (
        Scopes::Battle,
        "battle_side_pm_usage",
        Block(&[
            ("target", Scope(Scopes::Country)),
            ("production_method", Item(Item::ProductionMethod)),
            ("value", CompareValue),
        ]),
    ),
    (Scopes::Building, "building_has_goods_shortage", Boolean),
    (Scopes::Country, "bureaucracy", CompareValue),
    (Scopes::None, "calc_true_if", Control),
    (
        Scopes::State,
        "can_activate_production_method",
        Block(&[
            ("building_type", Item(Item::BuildingType)),
            ("production_method", Item(Item::ProductionMethod)),
        ]),
    ),
    (
        Scopes::Country,
        "can_afford_diplomatic_action",
        Block(&[("target", Scope(Scopes::Country)), ("type", Item(Item::DiplomaticAction))]),
    ),
    (Scopes::Character, "can_agitate", Scope(Scopes::Country)),
    (Scopes::Law, "can_be_enacted", Boolean),
    (Scopes::Country, "can_establish_any_export_route", ScopeOrItem(Scopes::Goods, Item::Goods)),
    (Scopes::Country, "can_establish_any_import_route", ScopeOrItem(Scopes::Goods, Item::Goods)),
    (Scopes::Country, "can_form_nation", Item(Item::Country)),
    (
        Scopes::Country,
        "can_have_as_subject",
        Block(&[("who", Scope(Scopes::Country)), ("type", Item(Item::SubjectType))]),
    ),
    (Scopes::Country, "can_have_subjects", Boolean),
    (Scopes::Building, "can_queue_building_levels", CompareValue),
    (Scopes::Country, "can_research", Item(Item::Technology)),
    (Scopes::None, "can_start_tutorial_lesson", Item(Item::TutorialLesson)),
    (
        Scopes::Country,
        "can_take_on_scaled_debt",
        Block(&[("who", Scope(Scopes::Country)), ("value", CompareValue)]),
    ),
    (Scopes::Building, "cash_reserves_available", CompareValue),
    (Scopes::Building, "cash_reserves_ratio", CompareValue),
    (Scopes::Character, "character_supports_political_movement", Boolean),
    (
        Scopes::Country
            .union(Scopes::Province)
            .union(Scopes::State)
            .union(Scopes::StateRegion)
            .union(Scopes::StrategicRegion)
            .union(Scopes::Theater),
        "check_area",
        Block(&[
            ("mode", Choice(&["adjacent"])),
            (
                "target",
                Scope(
                    Scopes::Country
                        .union(Scopes::Province)
                        .union(Scopes::State)
                        .union(Scopes::StateRegion)
                        .union(Scopes::StrategicRegion)
                        .union(Scopes::Theater),
                ),
            ),
        ]),
    ),
    (Scopes::CivilWar, "civil_war_progress", CompareValue),
    (Scopes::Character, "commander_is_available", Boolean),
    (
        Scopes::Character,
        "commander_pm_usage",
        Block(&[
            ("target", Scope(Scopes::Country)),
            ("production_method", Item(Item::ProductionMethod)),
            ("value", CompareValue),
        ]),
    ),
    (Scopes::Character, "commander_rank", CompareValue),
    (Scopes::Country, "construction_queue_duration", CompareValue),
    (Scopes::Country, "construction_queue_government_duration", CompareValue),
    (Scopes::Country, "construction_queue_num_queued_government_levels", CompareValue),
    (Scopes::Country, "construction_queue_num_queued_levels", CompareValue),
    (Scopes::Country, "construction_queue_num_queued_private_levels", CompareValue),
    (Scopes::Country, "construction_queue_private_duration", CompareValue),
    (Scopes::StateRegion, "contains_capital_of", ScopeOrItem(Scopes::Country, Item::Country)),
    (Scopes::CountryDefinition, "country_definition_has_culture", Scope(Scopes::Culture)),
    (Scopes::Country, "country_has_primary_culture", Scope(Scopes::Culture)),
    (Scopes::Country, "country_has_state_religion", Scope(Scopes::Religion)),
    (Scopes::Country, "country_or_subject_owns_entire_state_region", Item(Item::StateRegion)),
    (
        Scopes::Country,
        "country_pm_usage",
        Block(&[
            ("target", Scope(Scopes::Country)),
            ("production_method", Item(Item::ProductionMethod)),
            ("value", CompareValue),
        ]),
    ),
    (Scopes::Country, "country_rank", CompareValue),
    (Scopes::Country, "country_tier", ItemOrCompareValue(Item::CountryTier)),
    (Scopes::Pop, "culture_accepted", Boolean),
    (Scopes::Culture, "culture_is_discriminated_in", Scope(Scopes::Country)),
    (
        Scopes::Country,
        "culture_percent_country",
        Block(&[("target", Scope(Scopes::Culture)), ("value", CompareValue)]),
    ),
    (
        Scopes::State,
        "culture_percent_state",
        Block(&[("target", Scope(Scopes::Culture)), ("value", CompareValue)]),
    ),
    (
        Scopes::Culture,
        "culture_secession_progress",
        Block(&[("target", Scope(Scopes::Country)), ("value", CompareValue)]),
    ),
    (Scopes::None, "current_tooltip_depth", CompareValue),
    (Scopes::None, "custom_description", Control),
    (Scopes::None, "custom_tooltip", Special),
    (Scopes::None, "debug_log", UncheckedValue),
    (Scopes::None, "debug_log_details", Boolean),
    (Scopes::State, "devastation", CompareValue),
    (
        Scopes::DiplomaticPlay,
        "diplomatic_play_pm_usage",
        Block(&[
            ("target", Scope(Scopes::Country)),
            ("production_method", Item(Item::ProductionMethod)),
            ("value", CompareValue),
        ]),
    ),
    (Scopes::Building, "earnings", CompareValue),
    (Scopes::Party, "election_momentum", CompareValue),
    (Scopes::Country, "empty_agitator_slots", CompareValue),
    (Scopes::Country, "enacting_any_law", Boolean),
    (Scopes::Country, "enactment_chance", CompareValue),
    (Scopes::Country, "enactment_chance_without_enactment_modifier", CompareValue),
    (Scopes::Country, "enactment_phase", CompareValue),
    (Scopes::Country, "enactment_setback_count", CompareValue),
    (Scopes::None, "error_check", Special),
    (Scopes::DiplomaticPlay, "escalation", CompareValue),
    (Scopes::None, "exists", Special),
    (
        Scopes::Country,
        "expanding_institution",
        ScopeOrItem(Scopes::InstitutionType, Item::Institution),
    ),
    (Scopes::Character, "experience_level", CompareValue),
    (Scopes::State, "free_arable_land", CompareValue),
    (
        Scopes::Front,
        "front_side_pm_usage",
        Block(&[
            ("target", Scope(Scopes::Country)),
            ("production_method", Item(Item::ProductionMethod)),
            ("value", CompareValue),
        ]),
    ),
    (Scopes::None, "game_date", CompareDate),
    (Scopes::Country, "global_country_ranking", CompareValue),
    (
        Scopes::None,
        "global_variable_list_size",
        Block(&[("name", UncheckedValue), ("value", CompareValue)]),
    ),
    (Scopes::Country, "gold_reserves", CompareValue),
    (Scopes::Country, "gold_reserves_limit", CompareValue),
    (Scopes::Country, "government_legitimacy", CompareValue),
    (Scopes::Country, "government_transfer_of_power", Item(Item::TransferOfPower)),
    (Scopes::Country, "government_wage_level", CompareLevel),
    (Scopes::Country, "government_wage_level_value", CompareValue),
    (Scopes::Market.union(Scopes::State), "has_active_building", Item(Item::BuildingType)),
    (Scopes::Country, "has_active_peace_deal", Boolean),
    (Scopes::Building, "has_active_production_method", Item(Item::ProductionMethod)),
    (Scopes::Country, "has_any_secessionists_broken_out", Boolean),
    (Scopes::Country, "has_any_secessionists_growing", Boolean),
    (Scopes::Country, "has_any_secessionists_possible", Boolean),
    (Scopes::State, "has_assimilating_pops", Boolean),
    (
        Scopes::Country,
        "has_attitude",
        Block(&[("who", Scope(Scopes::Country)), ("attitude", Item(Item::Attitude))]),
    ),
    (Scopes::BattleSide, "has_battle_condition", Item(Item::BattleCondition)),
    (
        Scopes::Country.union(Scopes::Market).union(Scopes::State).union(Scopes::StateRegion),
        "has_building",
        Item(Item::BuildingType),
    ),
    (Scopes::Country, "has_claim", Scope(Scopes::State.union(Scopes::StateRegion))),
    (Scopes::State, "has_claim_by", Scope(Scopes::Country)),
    (Scopes::Character, "has_commander_order", Item(Item::CommanderOrder)),
    (Scopes::Country, "has_completed_subgoal", Item(Item::ObjectiveSubgoal)),
    (Scopes::Country, "has_consumption_tax", Scope(Scopes::Goods)),
    (Scopes::State, "has_converting_pops", Boolean),
    (Scopes::Country, "has_convoys_being_sunk", Boolean),
    (Scopes::None, "has_cosmetic_dlc", UncheckedValue),
    (Scopes::Culture, "has_cultural_obsession", Item(Item::Goods)),
    (Scopes::Character, "has_culture", Scope(Scopes::Character.union(Scopes::Culture))),
    (Scopes::Culture, "has_culture_graphics", Item(Item::CultureGraphics)),
    (Scopes::Country, "has_decreasing_interests", Boolean),
    (Scopes::State, "has_decree", Item(Item::Decree)),
    // TODO: limit the type to diplomatic actions that have pacts
    (
        Scopes::Country,
        "has_diplomatic_pact",
        Block(&[
            ("who", Scope(Scopes::Country)),
            ("type", Item(Item::DiplomaticAction)),
            ("?is_initiator", Boolean),
        ]),
    ),
    (Scopes::StrategicRegion, "has_diplomatic_play", Boolean),
    (Scopes::Country, "has_diplomatic_relevance", Scope(Scopes::Country)),
    (Scopes::Country, "has_diplomats_expelled", Scope(Scopes::Country)),
    (Scopes::Culture.union(Scopes::Religion), "has_discrimination_trait", UncheckedValue), // TODO
    (Scopes::None, "has_dlc_feature", Item(Item::DlcFeature)),
    (
        Scopes::Building,
        "has_employee_slots_filled",
        Block(&[("pop_type", Item(Item::PopType)), ("percent", CompareValue)]),
    ),
    (Scopes::Country, "has_export_priority_tariffs", ScopeOrItem(Scopes::Goods, Item::Goods)),
    (Scopes::Building, "has_failed_hires", Boolean),
    (Scopes::Country, "has_free_government_reform", Boolean),
    (Scopes::None, "has_game_rule", Item(Item::GameRuleSetting)),
    (Scopes::None, "has_game_started", Boolean),
    (Scopes::None, "has_gameplay_dlc", UncheckedValue),
    (Scopes::Country, "has_global_highest_gdp", Boolean),
    (Scopes::Country, "has_global_highest_innovation", Boolean),
    (Scopes::None, "has_global_variable", UncheckedValue),
    (Scopes::None, "has_global_variable_list", UncheckedValue),
    (Scopes::Country, "has_government_clout", CompareValue),
    (Scopes::Country, "has_government_type", Item(Item::GovernmentType)),
    (Scopes::Country, "has_healthy_economy", Boolean),
    (Scopes::Character, "has_high_attrition", Boolean),
    (Scopes::Culture, "has_homeland", Scope(Scopes::State.union(Scopes::StateRegion))),
    (
        Scopes::Character.union(Scopes::InterestGroup),
        "has_ideology",
        ScopeOrItem(Scopes::Ideology, Item::Ideology),
    ),
    (Scopes::Country, "has_import_priority_tariffs", ScopeOrItem(Scopes::Goods, Item::Goods)),
    (Scopes::Country, "has_institution", ScopeOrItem(Scopes::InstitutionType, Item::Institution)),
    (Scopes::Country, "has_insurrectionary_interest_groups", Boolean),
    (
        Scopes::Country,
        "has_interest_marker_in_region",
        ScopeOrItem(Scopes::StrategicRegion, Item::StrategicRegion),
    ),
    (Scopes::Country, "has_journal_entry", Item(Item::Journalentry)),
    (Scopes::Province, "has_label", Item(Item::TerrainLabel)),
    (Scopes::Country, "has_law", Scope(Scopes::LawType)),
    (Scopes::None, "has_local_variable", UncheckedValue),
    (Scopes::None, "has_local_variable_list", UncheckedValue),
    (Scopes::None, "has_map_interaction", UncheckedValue), // TODO
    (Scopes::None, "has_map_interaction_diplomatic_action", UncheckedValue), // TODO
    (Scopes::None, "has_map_interaction_export_goods", UncheckedValue), // TODO
    (Scopes::None, "has_map_interaction_import_goods", UncheckedValue), // TODO
    (
        Scopes::Country
            .union(Scopes::Building)
            .union(Scopes::Character)
            .union(Scopes::Front)
            .union(Scopes::Institution)
            .union(Scopes::InterestGroup)
            .union(Scopes::Journalentry)
            .union(Scopes::PoliticalMovement)
            .union(Scopes::State),
        "has_modifier",
        Item(Item::Modifier),
    ),
    (Scopes::Country, "has_no_priority_tariffs", ScopeOrItem(Scopes::Goods, Item::Goods)),
    (Scopes::Country, "has_objective", Item(Item::Objective)),
    (Scopes::Pop, "has_ongoing_assimilation", Boolean),
    (Scopes::Pop, "has_ongoing_conversion", Boolean),
    (Scopes::Country, "has_overlapping_interests", Scope(Scopes::Country)),
    (Scopes::InterestGroup, "has_party", Boolean),
    (Scopes::Party, "has_party_member", Scope(Scopes::InterestGroup)),
    (Scopes::DiplomaticPlay, "has_play_goal", Item(Item::Wargoal)),
    (Scopes::Pop, "has_pop_culture", Item(Item::Culture)),
    (Scopes::Pop, "has_pop_religion", Item(Item::Religion)),
    (Scopes::Country.union(Scopes::Market).union(Scopes::State), "has_port", Boolean),
    (Scopes::Country, "has_possible_decisions", Boolean),
    (Scopes::State, "has_potential_resource", Item(Item::BuildingGroup)),
    (Scopes::Country, "has_potential_to_form_country", UncheckedValue), // No examples in vanilla
    (Scopes::None, "has_reached_end_date", Boolean),
    (Scopes::Character, "has_religion", Scope(Scopes::Religion)),
    (Scopes::Country, "has_researchable_technology", Boolean),
    (Scopes::Country, "has_revolution", Boolean),
    (Scopes::Character, "has_role", Item(Item::CharacterRole)),
    (Scopes::Country, "has_ruling_interest_group", Item(Item::InterestGroup)),
    (Scopes::Country, "has_ruling_interest_group_count", CompareValue),
    (
        Scopes::Country,
        "has_secret_goal",
        Block(&[("who", Scope(Scopes::Country)), ("secret_goal", Item(Item::SecretGoal))]),
    ),
    (Scopes::Country, "has_state_in_state_region", Item(Item::StateRegion)),
    (Scopes::Pop, "has_state_religion", Boolean),
    (Scopes::State, "has_state_trait", Item(Item::StateTrait)),
    (Scopes::Country, "has_strategic_adjacency", Scope(Scopes::State.union(Scopes::Country))),
    (Scopes::Country, "has_strategic_land_adjacency", Scope(Scopes::State.union(Scopes::Country))),
    (Scopes::Country, "has_strategy", Item(Item::AiStrategy)),
    (Scopes::Country, "has_subject_relation_with", Scope(Scopes::Country)),
    (Scopes::Country, "has_sufficient_construction_capacity_for_investment", Boolean),
    // no examples in vanilla
    (
        Scopes::Country,
        "has_technology_progress",
        Block(&[("technology", UncheckedValue), ("progress", CompareValue)]),
    ),
    (Scopes::Country, "has_technology_researched", Item(Item::Technology)),
    (Scopes::Character, "has_template", Item(Item::CharacterTemplate)),
    (Scopes::Province, "has_terrain", Item(Item::Terrain)),
    (Scopes::Character, "has_trait", Item(Item::CharacterTrait)),
    (Scopes::Country, "has_treaty_port_in_country", Scope(Scopes::Country)),
    (Scopes::Country, "has_truce_with", Scope(Scopes::Country)),
    (Scopes::None, "has_unification_candidate", UncheckedValue), // No examples in vanilla
    (Scopes::None, "has_variable", UncheckedValue),
    (Scopes::None, "has_variable_list", UncheckedValue),
    (
        Scopes::War,
        "has_war_exhaustion",
        Block(&[("target", Scope(Scopes::Country)), ("value", CompareValue)]),
    ),
    (Scopes::War, "has_war_goal", Item(Item::Wargoal)),
    (
        Scopes::War,
        "has_war_support",
        Block(&[("target", Scope(Scopes::Country)), ("value", CompareValue)]),
    ),
    (Scopes::Country, "has_war_with", Scope(Scopes::Country)),
    (Scopes::Country, "has_wasted_construction", Boolean),
    (Scopes::None, "hidden_trigger", Control),
    (Scopes::Country, "highest_secession_progress", CompareValue),
    (Scopes::InterestGroup, "ig_approval", CompareApproval),
    (Scopes::InterestGroup, "ig_clout", CompareValue),
    (Scopes::InterestGroup, "ig_government_power_share", CompareValue),
    (
        Scopes::State,
        "ig_state_pol_strength_share",
        Block(&[("target", Scope(Scopes::InterestGroup)), ("value", CompareValue)]),
    ),
    (Scopes::Country, "in_default", Boolean),
    (Scopes::Country, "in_election_campaign", Boolean),
    (Scopes::State, "incorporation_progress", CompareValue),
    (Scopes::Country, "influence", CompareValue),
    (Scopes::State.union(Scopes::StateRegion), "infrastructure", CompareValue),
    (Scopes::State.union(Scopes::StateRegion), "infrastructure_usage", CompareValue),
    (Scopes::DiplomaticPlay, "initiator_is", Scope(Scopes::Country)),
    (
        Scopes::Country,
        "institution_investment_level",
        Block(&[("institution", Item(Item::Institution)), ("value", CompareValue)]),
    ),
    (Scopes::InterestGroup, "interest_group_population", CompareValue),
    (Scopes::InterestGroup, "interest_group_population_percentage", CompareValue),
    (Scopes::InterestGroup, "interest_group_supports_political_movement", Boolean),
    (Scopes::Country, "investment_pool", CompareValue),
    (Scopes::Country, "investment_pool_gross_income", CompareValue),
    (Scopes::Country, "investment_pool_net_income", CompareValue),
    // The docs say that is_adjacent takes a country, but testing shows it works with states too
    (Scopes::Country, "is_adjacent", Scope(Scopes::Country.union(Scopes::State))),
    (Scopes::Character, "is_advancing_on_front", Scope(Scopes::Front)),
    (Scopes::Country, "is_ai", Boolean),
    (Scopes::Country, "is_at_war", Boolean),
    (Scopes::Character, "is_attacker_in_battle", Boolean),
    (Scopes::Country, "is_banning_goods", ScopeOrItem(Scopes::Goods, Item::Goods)),
    (Scopes::InterestGroup, "is_being_bolstered", Boolean),
    (Scopes::InterestGroup, "is_being_suppressed", Boolean),
    (Scopes::Building, "is_buildable", Boolean),
    (Scopes::Building, "is_building_group", Item(Item::BuildingGroup)),
    (Scopes::Building, "is_building_type", Item(Item::BuildingType)),
    (
        Scopes::None,
        "is_building_type_expanded",
        ScopeOrItem(Scopes::BuildingType, Item::BuildingType),
    ),
    (Scopes::Character, "is_busy", Boolean),
    (Scopes::State, "is_capital", Boolean),
    (Scopes::Character, "is_character_alive", Boolean),
    (Scopes::CivilWar, "is_civil_war_type", Choice(&["revolution", "secession"])),
    (Scopes::State, "is_coastal", Boolean),
    (Scopes::Country, "is_construction_paused", Boolean),
    (Scopes::MarketGoods, "is_consumed_by_government_buildings", Boolean),
    (Scopes::MarketGoods, "is_consumed_by_military_buildings", Boolean),
    (Scopes::Country, "is_country_alive", Boolean),
    (Scopes::Country, "is_country_type", Item(Item::CountryType)),
    (Scopes::Character, "is_defender_in_battle", Boolean),
    (Scopes::DiplomaticPact, "is_diplomatic_action_type", Item(Item::DiplomaticAction)),
    (Scopes::DiplomaticPact, "is_diplomatic_in_danger", Boolean),
    (Scopes::Country, "is_diplomatic_play_committed_participant", Boolean),
    (Scopes::Country, "is_diplomatic_play_enemy_of", Scope(Scopes::Country)),
    (Scopes::Country, "is_diplomatic_play_initiator", Boolean),
    (Scopes::Country, "is_diplomatic_play_target", Boolean),
    (Scopes::DiplomaticPlay, "is_diplomatic_play_type", Item(Item::DiplomaticPlay)),
    (Scopes::Country, "is_diplomatic_play_undecided_participant", Boolean),
    (Scopes::Country, "is_direct_subject_of", Scope(Scopes::Country)),
    (Scopes::Pop, "is_employed", Boolean),
    (Scopes::Country, "is_enacting_law", Scope(Scopes::LawType)),
    (Scopes::Country, "is_expanding_institution", Boolean),
    (Scopes::Character, "is_female", Boolean),
    (Scopes::None, "is_game_paused", Boolean),
    (Scopes::None, "is_gamestate_tutorial_active", Boolean),
    (Scopes::Journalentry, "is_goal_complete", Boolean),
    (Scopes::Building, "is_government_funded", Boolean),
    (Scopes::Character, "is_heir", Boolean),
    (Scopes::Character, "is_historical", Boolean),
    (Scopes::Country, "is_home_country_for", Scope(Scopes::Country)),
    (Scopes::StateRegion, "is_homeland", ScopeOrItem(Scopes::Culture, Item::Culture)),
    (Scopes::State, "is_homeland_of_country_cultures", Scope(Scopes::Country)),
    (Scopes::Character, "is_in_battle", Boolean),
    (Scopes::Country, "is_in_customs_union", Boolean),
    (Scopes::Character, "is_in_exile_pool", Boolean),
    (Scopes::InterestGroup, "is_in_government", Boolean),
    (Scopes::None, "is_in_list", Special),
    (Scopes::State, "is_in_revolt", Boolean),
    (Scopes::Character, "is_in_void", Boolean),
    (Scopes::Country, "is_in_war_together", Scope(Scopes::Country)),
    (Scopes::State, "is_incorporated", Boolean),
    (Scopes::InterestGroup, "is_insurrectionary", Boolean),
    (Scopes::InterestMarker, "is_interest_active", Boolean),
    (
        Scopes::Character.union(Scopes::InterestGroup),
        "is_interest_group_type",
        Item(Item::InterestGroup),
    ),
    (Scopes::State, "is_isolated_from_market", Boolean),
    (Scopes::Country, "is_junior_in_customs_union", Boolean),
    (Scopes::Theater, "is_land_theater", Boolean),
    (Scopes::State, "is_largest_state_in_region", Boolean),
    (
        Scopes::None,
        "is_lens_open",
        Block(&[("lens", UncheckedValue), ("tab_name", UncheckedValue)]),
    ),
    (Scopes::Country, "is_local_player", Boolean),
    (Scopes::Country, "is_losing_power_rank", Boolean),
    (Scopes::InterestGroup, "is_marginal", Boolean),
    (Scopes::State, "is_mass_migration_target", Boolean),
    (Scopes::InterestGroup, "is_member_of_party", Scope(Scopes::Party)),
    (Scopes::Character, "is_mobilized", Boolean),
    (Scopes::Character, "is_monarch", Boolean),
    (Scopes::None, "is_objective_completed", Boolean),
    (Scopes::Character, "is_on_front", Boolean),
    (Scopes::Country, "is_owed_obligation_by", Scope(Scopes::Country)),
    (
        Scopes::None,
        "is_panel_open",
        Block(&[
            ("?target", UncheckedValue),
            ("?panel_name", UncheckedValue),
            ("tab_name", UncheckedValue),
        ]),
    ),
    (Scopes::Party, "is_party", Scope(Scopes::Party)),
    (Scopes::Party, "is_party_type", Item(Item::Party)),
    (Scopes::Country, "is_player", Boolean),
    (Scopes::PoliticalMovement, "is_political_movement_type", Item(Item::PoliticalMovement)),
    (Scopes::Pop, "is_pop_type", Item(Item::PopType)),
    (Scopes::None, "is_popup_open", UncheckedValue),
    (Scopes::InterestGroup, "is_powerful", Boolean),
    (Scopes::Culture, "is_primary_culture_of", Scope(Scopes::Country)),
    (
        Scopes::State,
        "is_production_method_active",
        Block(&[
            ("building_type", Item(Item::BuildingType)),
            ("production_method", Item(Item::ProductionMethod)),
        ]),
    ),
    (Scopes::Journalentry, "is_progressing", Boolean),
    (Scopes::Province, "is_province_land", Boolean),
    (Scopes::Character, "is_repairing", Boolean),
    (Scopes::Country, "is_researching_technology", Item(Item::Technology)), // TODO: also accepts "any"
    (Scopes::Country, "is_researching_technology_category", UncheckedValue), // No examples in vanilla
    (Scopes::Country.union(Scopes::InterestGroup), "is_revolutionary", Boolean),
    (Scopes::PoliticalMovement, "is_revolutionary_movement", Boolean),
    (Scopes::None, "is_rightclick_menu_open", Boolean),
    (Scopes::Character, "is_ruler", Boolean),
    (Scopes::InterestGroup, "is_same_interest_group_type", Scope(Scopes::InterestGroup)),
    (Scopes::LawType, "is_same_law_group_as", Scope(Scopes::LawType)),
    (Scopes::Party, "is_same_party_type", Scope(Scopes::Party)),
    (Scopes::State, "is_sea_adjacent", Boolean),
    (Scopes::Country, "is_secessionist", Boolean),
    (Scopes::None, "is_set", Special),
    (Scopes::State, "is_slave_state", Boolean),
    (Scopes::State, "is_split_state", Boolean),
    (Scopes::StateRegion, "is_state_region_land", Boolean),
    (Scopes::Religion, "is_state_religion", Scope(Scopes::Country)),
    (Scopes::State, "is_strategic_objective", Scope(Scopes::Country)),
    (Scopes::InterestGroup, "is_strongest_ig_in_government", Boolean),
    (Scopes::Country, "is_subject", Boolean),
    (Scopes::Country, "is_subject_of", Scope(Scopes::Country)),
    (Scopes::Country, "is_subject_type", Item(Item::SubjectType)),
    (Scopes::Building, "is_subsidized", Boolean),
    (Scopes::Building, "is_subsistence_building", Boolean),
    (
        Scopes::Country,
        "is_supporting_unification_candidate",
        Block(&[
            ("who", Scope(Scopes::Country)),
            ("country_formation", Item(Item::CountryFormation)),
        ]),
    ),
    (
        Scopes::None,
        "is_target_in_global_variable_list",
        Block(&[("name", UncheckedValue), ("*target", ScopeOkThis(Scopes::all_but_none()))]),
    ),
    (
        Scopes::None,
        "is_target_in_local_variable_list",
        Block(&[("name", UncheckedValue), ("*target", ScopeOkThis(Scopes::all_but_none()))]),
    ),
    (
        Scopes::None,
        "is_target_in_variable_list",
        Block(&[("name", UncheckedValue), ("*target", ScopeOkThis(Scopes::all_but_none()))]),
    ),
    (Scopes::State, "is_target_of_wargoal", Scope(Scopes::Country)),
    (Scopes::Country, "is_taxing_goods", ScopeOrItem(Scopes::Goods, Item::Goods)),
    (Scopes::TradeRoute, "is_trade_route_active", Boolean),
    (Scopes::TradeRoute, "is_trade_route_productive", Boolean),
    (Scopes::Goods.union(Scopes::MarketGoods), "is_tradeable", Boolean),
    (Scopes::Character, "is_traveling", Boolean),
    (Scopes::State, "is_treaty_port", Boolean),
    (Scopes::None, "is_tutorial_active", Boolean),
    (Scopes::None, "is_tutorial_lesson_active", Item(Item::TutorialLesson)),
    (Scopes::None, "is_tutorial_lesson_chain_completed", UncheckedValue), // TODO
    (Scopes::None, "is_tutorial_lesson_completed", Item(Item::TutorialLesson)),
    (Scopes::None, "is_tutorial_lesson_step_completed", UncheckedValue), // TODO
    (Scopes::State, "is_under_colonization", Boolean),
    (Scopes::Building, "is_under_construction", Boolean),
    (Scopes::Country, "is_unification_candidate", Item(Item::CountryFormation)),
    (Scopes::Country, "is_violating_sovereignty_of", Scope(Scopes::Country)),
    (Scopes::Front, "is_vulnerable_front", Scope(Scopes::Country)),
    (Scopes::DiplomaticPlay, "is_war", Boolean),
    (Scopes::War, "is_war_participant", Scope(Scopes::Country)),
    (Scopes::War, "is_warleader", Scope(Scopes::Country)),
    (Scopes::Country, "isolated_states", CompareValue),
    (Scopes::Law, "law_approved_by", Scope(Scopes::InterestGroup)),
    (
        Scopes::Character.union(Scopes::InterestGroup),
        "law_stance",
        Block(&[("law", Scope(Scopes::LawType)), ("value", CompareStance)]),
    ),
    (Scopes::Country, "leading_producer_of", Scope(Scopes::Goods)),
    (Scopes::Country, "leads_customs_union", Boolean),
    (Scopes::None, "list_size", Block(&[("name", UncheckedValue), ("value", CompareValue)])),
    (Scopes::Country.union(Scopes::Pop).union(Scopes::State), "literacy_rate", CompareValue),
    (
        Scopes::None,
        "local_variable_list_size",
        Block(&[("name", UncheckedValue), ("value", CompareValue)]),
    ),
    (
        Scopes::Country.union(Scopes::State),
        "loyalist_fraction",
        Block(&[
            ("value", CompareValue),
            ("?pop_type", Item(Item::PopType)),
            ("?strata", Item(Item::Strata)),
            ("?culture", ScopeOrItem(Scopes::Culture, Item::Culture)),
            ("?religion", ScopeOrItem(Scopes::Religion, Item::Religion)),
        ]),
    ),
    (Scopes::State, "loyalty", CompareValue),
    (Scopes::State, "market_access", CompareValue),
    (Scopes::MarketGoods, "market_goods_buy_orders", CompareValue),
    (Scopes::MarketGoods, "market_goods_cheaper", CompareValue),
    (Scopes::MarketGoods, "market_goods_consumption", CompareValue),
    (Scopes::MarketGoods, "market_goods_delta", CompareValue),
    (Scopes::MarketGoods, "market_goods_exports", CompareValue),
    (Scopes::MarketGoods, "market_goods_has_goods_shortage", Boolean),
    (Scopes::MarketGoods, "market_goods_imports", CompareValue),
    (Scopes::MarketGoods, "market_goods_pricier", CompareValue),
    (Scopes::MarketGoods, "market_goods_production", CompareValue),
    (Scopes::MarketGoods, "market_goods_sell_orders", CompareValue),
    (Scopes::MarketGoods, "market_goods_shortage_ratio", CompareValue),
    (Scopes::Market, "market_has_goods_shortage", Boolean),
    (Scopes::Country, "max_num_declared_interests", CompareValue),
    (Scopes::Country, "military_wage_level", CompareLevel),
    (Scopes::Country, "military_wage_level_value", CompareValue),
    (Scopes::Character, "mobilization_cost", CompareValue),
    (Scopes::InterestGroup, "most_powerful_strata", Item(Item::Strata)),
    (Scopes::State, "most_prominent_revolting_interest_group", Item(Item::InterestGroup)),
    (Scopes::None, "nand", Control),
    (Scopes::Country, "navy_reserves", CompareValue),
    (Scopes::None, "nor", Control),
    (Scopes::None, "not", Control),
    (Scopes::Country, "num_alliances_and_defensive_pacts_with_allies(", CompareValue),
    (Scopes::Country, "num_alliances_and_defensive_pacts_with_rivals(", CompareValue),
    (Scopes::War, "num_casualties", CompareValue),
    (
        Scopes::War,
        "num_country_casualties",
        Block(&[("target", Scope(Scopes::Country)), ("value", CompareValue)]),
    ),
    (
        Scopes::War,
        "num_country_dead",
        Block(&[("target", Scope(Scopes::Country)), ("value", CompareValue)]),
    ),
    (
        Scopes::War,
        "num_country_wounded",
        Block(&[("target", Scope(Scopes::Country)), ("value", CompareValue)]),
    ),
    (Scopes::War, "num_dead", CompareValue),
    (Scopes::Country, "num_declared_interests", CompareValue),
    (Scopes::Front, "num_defending_battalions(", CompareValue),
    (Scopes::Front, "num_enemy_units(", CompareValue),
    (Scopes::Country, "num_mutual_trade_route_levels_with_country(", CompareValue),
    (Scopes::Country, "num_taxed_goods", CompareValue),
    (Scopes::Front, "num_total_battalions(", CompareValue),
    (Scopes::War, "num_wounded", CompareValue),
    (Scopes::Country, "number_of_possible_decisions", CompareValue),
    (Scopes::Building, "occupancy", CompareValue),
    (Scopes::None, "or", Control),
    (Scopes::Country, "owes_obligation_to", Scope(Scopes::Country)),
    (
        Scopes::Country,
        "owns_entire_state_region",
        ScopeOrItem(Scopes::StateRegion, Item::StateRegion),
    ),
    (Scopes::Country, "owns_treaty_port_in", ScopeOrItem(Scopes::StateRegion, Item::StateRegion)),
    (Scopes::PoliticalMovement, "political_movement_radicalism", CompareValue),
    (Scopes::PoliticalMovement, "political_movement_support", CompareValue),
    (Scopes::StateRegion, "pollution_amount", CompareValue),
    (Scopes::State, "pollution_generation", CompareValue),
    (Scopes::Pop, "pop_employment_building", Item(Item::BuildingType)),
    (Scopes::Pop, "pop_employment_building_group", Item(Item::BuildingGroup)),
    (Scopes::Pop, "pop_has_primary_culture", Boolean),
    (Scopes::Pop, "pop_is_discriminated", Boolean),
    (
        Scopes::Country,
        "pop_type_percent_country",
        Block(&[("pop_type", UncheckedValue), ("percent", CompareValue)]),
    ),
    (
        Scopes::InterestGroup,
        "prefers_law",
        Block(&[("law", Scope(Scopes::LawType)), ("comparison_law", Scope(Scopes::LawType))]),
    ),
    (Scopes::Country, "prestige", CompareValue),
    (Scopes::Country, "produced_authority", CompareValue),
    (Scopes::Country, "produced_bureaucracy", CompareValue),
    (Scopes::Country, "produced_influence", CompareValue),
    (Scopes::Pop, "quality_of_life", CompareValue),
    (
        Scopes::Country.union(Scopes::State),
        "radical_fraction",
        Block(&[
            ("value", CompareValue),
            ("?pop_type", Item(Item::PopType)),
            ("?strata", Item(Item::Strata)),
            ("?culture", ScopeOrItem(Scopes::Culture, Item::Culture)),
            ("?religion", ScopeOrItem(Scopes::Religion, Item::Religion)),
        ]),
    ),
    (Scopes::State, "relative_infrastructure", CompareValue),
    (Scopes::Pop, "religion_accepted", Boolean),
    (
        Scopes::Country,
        "pop_type_percent_country",
        Block(&[("target", Scope(Scopes::Religion)), ("value", CompareValue)]),
    ),
    (
        Scopes::State,
        "pop_type_percent_state",
        Block(&[("target", Scope(Scopes::Religion)), ("value", CompareValue)]),
    ),
    (Scopes::Country, "relations(", CompareValue),
    (
        Scopes::StateRegion,
        "remaining_undepleted",
        Block(&[("type", Item(Item::BuildingGroup)), ("amount", CompareValue)]),
    ),
    (Scopes::Country, "ruler_can_have_command", Boolean),
    (Scopes::all_but_none(), "save_temporary_scope_as", Special),
    (Scopes::None, "save_temporary_scope_value_as", Special),
    (Scopes::Country, "scaled_debt", CompareValue),
    (
        Scopes::Culture,
        "shares_heritage_and_other_trait_with_any_primary_culture",
        Scope(Scopes::Country),
    ),
    (Scopes::Culture, "shares_heritage_trait_with_any_primary_culture", Scope(Scopes::Country)),
    (Scopes::Religion, "shares_heritage_trait_with_state_religion", Scope(Scopes::Country)),
    (Scopes::Culture, "shares_non_heritage_trait_with_any_primary_culture", Scope(Scopes::Country)),
    (Scopes::Culture, "shares_trait_with_any_primary_culture", Scope(Scopes::Country)),
    (Scopes::Religion, "shares_trait_with_state_religion", Scope(Scopes::Country)),
    (Scopes::Country, "should_set_wargoal", Boolean),
    (Scopes::None, "should_show_nudity", Boolean),
    (Scopes::Country, "shrinking_institution", Item(Item::Institution)),
    (Scopes::Pop, "standard_of_living", CompareValue),
    (Scopes::State, "state_has_goods_shortage", Boolean),
    (Scopes::State, "state_population", CompareValue),
    (Scopes::State, "state_unemployment_rate", CompareValue),
    (Scopes::Pop, "strata", CompareValue),
    (Scopes::Country, "supply_network_strength", CompareValue),
    (Scopes::None, "switch", Special),
    (Scopes::Country, "taking_loans", Boolean),
    (Scopes::DiplomaticPlay, "target_is", Scope(Scopes::Country)),
    (Scopes::State, "tax_capacity", CompareValue),
    (Scopes::State, "tax_capacity_usage", CompareValue),
    (Scopes::Country, "tax_level", CompareLevel),
    (Scopes::Country, "tax_level_value", CompareValue),
    (Scopes::Country, "tension(", CompareValue),
    (Scopes::Country, "total_population", CompareValue),
    (Scopes::State, "total_urbanization", CompareValue),
    (Scopes::TradeRoute, "trade_route_needs_convoys_to_grow", Boolean),
    (Scopes::Character, "trait_value", CompareValue),
    (Scopes::None, "trigger_else", Control),
    (Scopes::None, "trigger_else_if", Control),
    (Scopes::None, "trigger_if", Control),
    (Scopes::State, "turmoil", CompareValue),
    // docs for all three say "target" instead of "value"
    (
        Scopes::None,
        "variable_list_size",
        Block(&[("name", UncheckedValue), ("value", CompareValue)]),
    ),
    (Scopes::War, "war_has_active_peace_deal", Boolean),
    (Scopes::Character, "was_exiled", Boolean),
    (Scopes::Country, "was_formed_from", Item(Item::Country)),
    (Scopes::Pop, "wealth", CompareValue),
    (Scopes::Country, "weekly_net_fixed_income", CompareValue),
    (Scopes::Building, "weekly_profit", CompareValue),
    (Scopes::None, "weighted_calc_true_if", Special),
    (Scopes::None, "year", CompareValue),
];
