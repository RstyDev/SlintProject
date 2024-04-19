import { invoke } from "@tauri-apps/api/tauri";



var error = new Audio('./../../assets/error.mp3');


function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}
async function submitea(e) {
    e.preventDefault();
    const id = document.getElementById('id');
    const pass = document.getElementById('pass');
    console.log(id.value);
    console.log(pass.value);
    try {
        await invoke("try_login", { id: id.value, pass: pass.value });
    } catch (err) {
        console.log(err)
        error.play();
        if (err.includes("Usuario")) {
            id.classList.add("error");
            setTimeout(() => { id.classList.toggle("error") }, 1000)
        } else if (err.includes("Contraseña")) {
            pass.classList.add("error");
            setTimeout(() => { pass.classList.toggle("error") }, 1000)
        }
    }
}

function Form() {

    //id.focus();

    return (
        <form onSubmit={submitea} id="form-login">
            <input type="text" autoFocus tabIndex={0} name="Id" id="id" placeholder="Usuario" autoComplete="off" required>
            </input>
            <input type="password" name="Password" id="pass" placeholder="Contraseña" required>
            </input>
            <input type="submit" value="Iniciar sesión">
            </input>
        </form>
    );
}

export default Form;