pub enum ValidationError {
    EmptyPlan,
    TooManyFilesInBundle { max: u32 },
    DiscoveryNotLoaded,
    ProfileTierMismatch,
}

pub type ValidationResult<T> = Result<T, ValidationError>;

pub fn validate_discovery_contract(dc: &DiscoveryContract) -> ValidationResult<()> {
    if dc.spineRefs.invariantsSpineId.is_empty()
        || dc.spineRefs.entertainmentMetricsSpineId.is_empty()
        || dc.spineRefs.schemaSpineIndexId.is_empty()
    {
        return Err(ValidationError::DiscoveryNotLoaded);
    }
    Ok(())
}

pub fn validate_authoring_contract(ac: &AiSafeAuthoringContract) -> ValidationResult<()> {
    if ac.plan.artifacts.is_empty() {
        return Err(ValidationError::EmptyPlan);
    }

    for art in &ac.plan.artifacts {
        if art.maxFilesInBundle == 0 || art.maxFilesInBundle > 3 {
            return Err(ValidationError::TooManyFilesInBundle { max: art.maxFilesInBundle });
        }
    }

    Ok(())
}
