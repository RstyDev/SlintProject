import "./Form.css"
import { useState } from "react";
function Form(){
    const [credito,setCredito] = useState(<></>);
    
    return (
    <form className="add-form">
        <input type="text" name="nombre" id="nombre" placeholder="Nombre" required/>
        <input type="number" name="dni" id="dni" placeholder="DNI" required/>
        <article>
            <p id="cuenta">Cuenta Corriente: </p>
            <input type="checkbox" name="credito"  id="credito" onChange={(e)=>{console.log(credito);setCredito(e.currentTarget.checked?<>
            <input type="number" placeholder="Límite" name="limite" id="limite" required step="0.01" />
            </>:<></>)}}/>
        </article>
        {credito}
    </form>
    )
}

export default Form;