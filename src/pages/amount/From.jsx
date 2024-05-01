export default function Form(){
    return(<form>
        <label htmlFor="monto">Monto:</label>
        <input type="number" name="monto" step={0.01}/>
    </form>)
}