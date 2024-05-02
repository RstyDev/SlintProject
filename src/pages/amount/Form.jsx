import { useState } from "react"
import { listen } from "@tauri-apps/api/event"
export default function Form(){
    const [state,setState]=useState({prod:{},rend:<></>})
    async function unlisten() {
        return await listen('main', (pl) => {
            console.log(pl)
            if(pl.payload.message=='select-amount'){
                setState({prod:pl.payload.val,rend:(Object.keys(pl.payload.val)[0]=='Rub')?
                <form>
                    <label htmlFor="monto">Monto:</label>
                    <input type="number" name="monto" step={0.01}/>
                </form>:
                <form>
                    <label htmlFor="peso">Peso:</label>
                    <input type="number" name="peso" step={0.01} />
                </form>
            })
            }  
        })
    }
    unlisten();
    return(state.rend)
}