const { invoke } = window.__TAURI__.tauri;
var error = new Audio('./../assets/error.mp3');
const id=document.getElementById('id');
const pass=document.getElementById('pass');


document.getElementById('form-login').addEventListener('submit',async (e)=>{
  e.preventDefault();
  try{
    await invoke("try_login", {id:id.value,pass:pass.value});
  }catch (err){
    console.log(err)
    error.play();
    if (err.includes("Usuario")){
      id.classList.add("error");
      setTimeout(() => { id.classList.toggle("error") }, 1000)
    }else if(err.includes("Contraseña")){
      pass.classList.add("error");
      setTimeout(() => { pass.classList.toggle("error") }, 1000)
    }
  }
})