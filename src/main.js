const { invoke } = window.__TAURI__.tauri;

let card_section;

window.addEventListener("DOMContentLoaded", () => {
  card_section = document.querySelector("#card_section");
  get_movies();
});

async function get_movies()
{
  document.getElementById("card_section").innerHTML = await invoke("get_movies_cards");
  console.log("working!");
}

window.get_movies = get_movies;
