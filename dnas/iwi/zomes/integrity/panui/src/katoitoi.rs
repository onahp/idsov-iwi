use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Katoitoi {
    pub katoitoi: String,
    pub panui_hash: ActionHash,
}
pub fn validate_create_katoitoi(
    _action: EntryCreationAction,
    katoitoi: Katoitoi,
) -> ExternResult<ValidateCallbackResult> {
    let record = must_get_valid_record(katoitoi.panui_hash.clone())?;
    let _panui: crate::Panui = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Dependant action must be accompanied by an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_katoitoi(
    _action: Update,
    _katoitoi: Katoitoi,
    _original_action: EntryCreationAction,
    _original_katoitoi: Katoitoi,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Katoitois cannot be updated")))
}
pub fn validate_delete_katoitoi(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_katoitoi: Katoitoi,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_panui_to_katoitois(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(base_address);
    let record = must_get_valid_record(action_hash)?;
    let _panui: crate::Panui = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _katoitoi: crate::Katoitoi = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_panui_to_katoitois(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("PanuiToKatoitois links cannot be deleted"),
        ),
    )
}
