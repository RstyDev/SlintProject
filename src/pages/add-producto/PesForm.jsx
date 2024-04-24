import { useState } from "react";

function PesForm(){
    const [porc, setPorc] = useState(40);
    const [costo, setCosto] = useState(0);
    const [precio, setPrecio] = useState(0);
    return(
        <form>
            <input type="text" placeholder="Descripción" required/>
            <input type="number" placeholder="Costo por kilo" onChange={(e)=>{
                setCosto(e.currentTarget.value);
                setPrecio(e.currentTarget.value * (1+(porc/100)))
                }} defaultValue={costo} step={0.01}/>
            <input type="number" placeholder="Porcentaje" onChange={(e)=>{
                setPorc(e.currentTarget.value);
                setPrecio(costo * (1+(e.currentTarget.value/100)));
            }} defaultValue={porc} step={0.01} />
            <input type="number" onChange={(e)=>{
                setPrecio(e.currentTarget.value);
                setPorc(((e.currentTarget.value/costo)-1)*100)
            }} placeholder="Precio por kilo" defaultValue={precio} step={0.01} required/>
            <input type="number" placeholder="Código" />
        </form>
    )
}

export default PesForm;