pub mod output {
    pub const TERR_DIST_IDX: usize = 0;
    pub const ALT_IDX: usize = TERR_DIST_IDX + 1;
    pub const HDG_IDX: usize = ALT_IDX + 1;
    pub const PITCH_IDX: usize = HDG_IDX + 1;
    pub const ROLL_IDX: usize = PITCH_IDX + 1;
    pub const AIL_IDX: usize = ROLL_IDX + 1;
    pub const ELEV_IDX: usize = AIL_IDX + 1;
    pub const RUD_IDX: usize = ELEV_IDX + 1;
    pub const FLP_IDX: usize = RUD_IDX + 1;
    pub const ENG_L_IDX: usize = FLP_IDX + 1;
    pub const ENG_R_IDX: usize = ENG_L_IDX + 1;
    pub const GEAR_F_IDX: usize = ENG_R_IDX + 1;
    pub const GEAR_L_IDX: usize = GEAR_F_IDX + 1;
    pub const GEAR_R_IDX: usize = GEAR_L_IDX + 1;
    pub const LIGHTS_IDX: usize = GEAR_R_IDX + 1;
}
