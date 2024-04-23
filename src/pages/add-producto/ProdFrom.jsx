import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Codes from "./Codes";

function ProdForm(){
    const [codes, setCodes] = useState([]);
    const [porc, setPorc] = useState(40);
    const [costo, setCosto] = useState(0);
    const [precio, setPrecio] = useState(0);
    return(<>
        <form> 
            <input type="text" placeholder="Tipo de Producto"/>
            <input type="text" placeholder="Marca" />
            <input type="text" placeholder="Variedad"/>
            <input type="number" placeholder="Cantidad" />
            <select name="presentacion"> 
                <option value="Gr">Gr</option>
                <option value="Un">Un</option>
                <option value="Lt">Lt</option>
                <option value="Ml">Ml</option>
                <option value="CC">CC</option>
                <option value="Kg">Kg</option>
            </select>
            <input type="number" step="0.01" onChange={(e)=>{
                setCosto(e.currentTarget.value);
                setPrecio(e.currentTarget.value * (1+(porc/100)))
                }} defaultValue={costo} placeholder="Costo"/>
            <input type="number" step="0.01" onChange={(e)=>{
                setPorc(e.currentTarget.value);
                setPrecio(costo * (1+(e.currentTarget.value/100)));
            }} defaultValue={porc} placeholder="Porcentaje"/>
            <input type="number" step="0.01" onChange={(e)=>{
                setPrecio(e.currentTarget.value);
                setPorc(((e.currentTarget.value/costo)-1)*100)
            }} defaultValue={precio} placeholder="Precio de Venta"/>
        </form>
            <Codes codes={codes} setCodes={setCodes}/>
    </>)
}

export default ProdForm;