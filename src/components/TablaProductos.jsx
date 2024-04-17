import ProductoBusqueda from "./ProductoBusqueda";
import { useEffect, useState } from "react";
import "./TablaProductos.css"

function TablaProductos({ conf, productos,focuseado }) {
    const [focused, setFocused] = useState(focuseado);
    useEffect(()=>{setFocused(focuseado)},[focuseado])
    function mapProds() {
        return productos.map(function (prod, i) {
            return <ProductoBusqueda key={i} conf={conf} producto={prod} focused={focused == i ? "focuseado" : ""} />
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