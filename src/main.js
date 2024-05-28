const { invoke } = window.__TAURI__.tauri;


let greetMsgEl;

async function goToWebsite() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  window.location.replace("http://monad360.com/home/tabs")
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    goToWebsite();
  });
});

//document.addEventListener('contextmenu', event => event.preventDefault());