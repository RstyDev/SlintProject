import ProductoBusqueda from "./ProductoBusqueda";
import { useEffect, useState } from "react";
import "./TablaProductos.css"

function TablaProductos({ conf, productos,focuseado,setFocuseado,pos,draw }) {
    const [focused, setFocused] = useState(focuseado);
    useEffect(()=>{setFocused(focuseado)},[focuseado])
    function mapProds() {
        return productos.map(function (prod, i) {
            return <ProductoBusqueda draw={draw} key={i} pos={pos} conf={conf} producto={prod} index={i} setFocuseado={setFocuseado} focused={focused == i ? "focuseado" : ""} />
        })
    }
    return (<table id="tabla-productos">
        <thead>
            <tr>
                <td>Producto</td>
                <td>Precio</td>
            </tr>
        </thead>
        <tbody>
            {mapProds()}
        </tbody>
    </table>)
}

export default TablaProductos;