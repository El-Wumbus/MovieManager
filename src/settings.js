const { invoke } = window.__TAURI__.tauri;
async function add_directory() {
    let folder = document.getElementById("path").innerText;
    let results = await invoke("add_lib", { path: folder });

    if (results != "") {
        console.log("Error: Couldn't add lib '" + folder + "'");
    }
    update_list();
}

async function remove_directory() {
    let folder = document.getElementById("path".innerText);
    let results = await invoke("remove_lib", { path: folder });

    if (results != "") {
        console.log("Error: Couldn't remove lib '" + folder + "'");
    }
    update_list();
}

async function update_list() {
    let list = await invoke("list_lib");
    document.getElementById("lib_folders").innerText = list;
}