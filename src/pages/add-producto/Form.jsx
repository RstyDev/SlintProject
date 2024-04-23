import { useState } from "react";
import ProdForm from "./ProdFrom";
import PesForm from "./PesForm";

function Form(){
    const prodForm = <ProdForm />
    const pesForm = <PesForm />
    const [tipo,setTipo] = useState(0);

    return(<>
        <select name="tipo" id="tipo" onChange={(e)=>setTipo(e.currentTarget.value)}>
            <option value="0">Producto</option>
            <option value="1">Pesable</option>
            <option value="2">Rubro</option>
        </select>
    </>)
}

export default Form;