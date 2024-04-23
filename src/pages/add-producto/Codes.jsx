import { useState } from "react";
import { useEffect } from "react";
function Codes({codes,setCodes}){
    const [cods, setCods] = useState(codes);
    useEffect(()=>{setCods(codes.map(function (code,i){
        return <>
            <input disabled={true} defaultValue={code}/>
            <button onClick={setCodes([...codes].splice(i,1))}>Borrar</button>
        </>
    }))},[codes]);
    return (<>
        <span>Se recomienda no agregar más de 3 productos por producto</span>
        {cods}
        <form onSubmit={(e)=>setCodes([...codes,e.currentTarget.value])}>
            <input type="number" name="code" placeholder="Código" id="code"/>
            <button type="submit"></button>
        </form>

    </>)
}

export default Codes;