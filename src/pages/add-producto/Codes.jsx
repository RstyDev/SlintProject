import { useState } from "react";
import { useEffect } from "react";
function mapea(codes,rm){
    return codes.map(function(code,i){
        return <article key={i}>
        <input disabled={true} value={code}/>
        <button onClick={()=>{rm(i)}}>Borrar</button>
    </article>
    })
}
function Codes({codes,setCodes}){
    const [cods,setCods] = useState(codes);
    const [inputs,setInputs] = useState(mapea(codes,rm))
    useEffect(()=>{setCods(codes)},[codes])
    useEffect(()=>{setInputs(mapea(codes,rm))},[cods])
    function rm(index){
        let codigos=[...cods];
        codigos.splice(index,1);
        setCodes(codigos);
    }
    const [rend,setRend]=useState(<>
        <span>Se recomienda no agregar más de 3 productos por producto</span>
        {inputs}
        <section >
            <input onKeyDown={(e)=>{console.log(e.key);if(e.keyCode==13){setCodes([...cods,e.currentTarget.value])}}} type="number" name="code" placeholder="Código" id="code"/>
            <button onClick={(e)=>setCodes([...cods,e.currentTarget.previousElementSibling.value])} >Agregar</button>
        </section>
    </>);
    useEffect(()=>{setRend(<>
        <span>Se recomienda no agregar más de 3 productos por producto</span>
        {inputs}
        <section >
            <input onKeyDown={(e)=>{console.log(e.key);if(e.keyCode==13){setCodes([...cods,e.currentTarget.value])}}} type="number" name="code" placeholder="Código" id="code"/>
            <button onClick={(e)=>setCodes([...cods,e.currentTarget.previousElementSibling.value])} >Agregar</button>
        </section>
    </>)},[inputs])
    return (rend)
}

export default Codes;