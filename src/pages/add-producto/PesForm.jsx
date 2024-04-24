import { useState } from "react";

function PesForm(){
    const [porc, setPorc] = useState(40);
    const [costo, setCosto] = useState(0);
    const [precio, setPrecio] = useState(0);
    return(
        <form>
            <label htmlFor="desc">Descripción:</label>
            <input name="desc" type="text" placeholder="Descripción" required/>
            <label htmlFor="costo">Costo:</label>
            <input type="number" name="costo" placeholder="Costo por kilo" onChange={(e)=>{
                setCosto(e.currentTarget.value);
                setPrecio(e.currentTarget.value * (1+(porc/100)))
                }} value={costo} step={0.01}/>
            <label htmlFor="porc">Porcentaje:</label>
            <input type="number" name="porc" placeholder="Porcentaje" onChange={(e)=>{
                setPorc(e.currentTarget.value);
                setPrecio(costo * (1+(e.currentTarget.value/100)));
            }} value={porc} step={0.01} />
            <label htmlFor="precio">Precio:</label>
            <input type="number" name="precio" onChange={(e)=>{
                setPrecio(e.currentTarget.value);
                setPorc(((e.currentTarget.value/costo)-1)*100)
            }} placeholder="Precio por kilo" value={precio} step={0.01} required/>
            <label htmlFor="cod">Código</label>
            <input type="number" name="cod" placeholder="Código" />
        </form>
    )
}

export default PesForm;