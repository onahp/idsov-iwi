use hdk::prelude::*;
use panui_integrity::*;

#[hdk_extern]
pub fn create_panui(panui: Panui) -> ExternResult<Record> {
    let panui_hash = create_entry(&EntryTypes::Panui(panui.clone()))?;
    let record = get(panui_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Panui"))
            ),
        )?;
    let path = Path::from("panui_katoa");
    create_link(path.path_entry_hash()?, panui_hash.clone(), LinkTypes::PanuiKatoa, ())?;
    Ok(record)
}

#[hdk_extern]
pub fn get_panui(original_panui_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(original_panui_hash.clone(), LinkTypes::PanuiUpdates, None)?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_panui_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_panui_hash.clone(),
    };
    get(latest_panui_hash, GetOptions::default())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePanuiInput {
    pub original_panui_hash: ActionHash,
    pub previous_panui_hash: ActionHash,
    pub updated_panui: Panui,
}

#[hdk_extern]
pub fn update_panui(input: UpdatePanuiInput) -> ExternResult<Record> {
    let updated_panui_hash = update_entry(
        input.previous_panui_hash.clone(),
        &input.updated_panui,
    )?;
    create_link(
        input.original_panui_hash.clone(),
        updated_panui_hash.clone(),
        LinkTypes::PanuiUpdates,
        (),
    )?;
    let record = get(updated_panui_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Panui"))
            ),
        )?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_panui(original_panui_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_panui_hash)
}
