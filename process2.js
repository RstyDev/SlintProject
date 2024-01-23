function elegirAleatoriamente(lista) {
    var indiceAleatorio = Math.floor(Math.random() * lista.length);
    return lista[indiceAleatorio];
}
function getCode() {
    return (Math.floor(Math.random() * 99999999999999))
}

function getCostPr() {
    return (Math.random() * 10000)
}
function getSalePr(piso) {
    
    return piso*(1+Math.random())
    
}

algo = {
    "id": 943,
    "codigos_de_barras": [
        60887106094103
    ],
    "precio_de_venta": 450.0,
    "porcentaje": 167.85714285714283,
    "precio_de_costo": 168.0,
    "tipo_producto": "Gaseosa",
    "marca": "Coca",
    "variedad": "Cola",
    "presentacion": {
        "Ml": 500
    }
}

class Producto {
    constructor(
        id, codigos_de_barras, precio_de_venta, precio_de_costo, tipo_producto, marca, variedad, presentacion, cantidad) {
        this.id = id;
        this.codigos_de_barras = [];
        for (let i = 0; i < codigos_de_barras.length; i++) {
            this.codigos_de_barras.push(codigos_de_barras[i]);
        }
        this.precio_de_venta = precio_de_venta;
        this.porcentaje = (precio_de_venta / precio_de_costo - 1) * 100
        this.precio_de_costo = precio_de_costo;
        this.tipo_producto = tipo_producto;
        this.marca = marca;
        this.variedad = variedad;
        switch (presentacion) {
            case 'Gr':
                this.presentacion = {
                    Gr: cantidad
                }
                break;
            case 'Un':
                this.presentacion = {
                    Un: cantidad
                }
                break;
            case 'Lt':
                this.presentacion = {
                    Lt: cantidad
                }
                break;
            case 'Ml':
                this.presentacion = {
                    Ml: cantidad
                }
                break;
            case 'CC':
                this.presentacion = {
                    CC: cantidad
                }
                break;
            case 'Kg':
                this.presentacion = {
                    Kg: cantidad
                }
                break;
        }

    }
}

let posact = 1462;
cantCodes = [1, 2, 3];
marcas_chocolates = [
    "Hershey's",
    "Cadbury",
    "Godiva",
    "Lindt",
    "Ferrero Rocher",
    "Ghirardelli",
    "Toblerone",
    "Nestlé",
    "Milka",
    "Mars",
    "Kinder",
    "Green & Black's",
    "Perugina",
    "Ritter Sport",
    "Valrhona",
    "Guylian",
    "Taza Chocolate",
    "Callebaut",
    "Fazer",
    "Terry's Chocolate Orange"
];
variedades_chocolate = [
    "Blanco",
    "Negro",
    "Relleno Frutilla",
    "Dulce De Leche",
    "Almendras",
    "Mani"
];
cantidades_chocolates = [50, 100, 150, 180, 250, 500]
marcas_arroz = [
    "Uncle Ben's",
    "Basmati",
    "Jasmine",
    "Arroz Carolina",
    "Mahatma",
    "Tilda",
    "SunRice",
    "Blue Ribbon",
    "Nishiki",
    "Kokuho",
    "Lundberg",
    "Roso",
    "Golden Star",
    "Zafarani",
    "Royal",
    "Kohinoor",
    "Annie Chun's",
    "Bombay Market",
    "Riceland",
    "Texana"
]
variedades_arroz = [
    "Largo Fino",
    "Parboil",
    "Doble Carolina",
    "Yamani"
]
cantidades_arroz = [250, 500, 1000];
marcas_galletitas = [
    "Nabisco",
    "Keebler",
    "Pepperidge Farm",
    "Arnott's",
    "McVitie's",
    "Lotus Biscoff",
    "Walkers Shortbread",
    "Oreo",
    "Girl Scout Cookies",
    "Leibniz",
    "LU",
    "Royal Dansk",
    "Famous Amos",
    "Belvita",
    "Tate's Bake Shop",
    "Voortman",
    "Carr's",
    "Ritz",
    "Mary's Gone Crackers",
    "Lorna Doone"
];
variedades_galletitas = ['Chocolate', 'Surtidas', 'Vainilla', 'Limon', 'Glaseadas', 'Rellenas Chocolate']
cantidades_galletitas = [150, 170, 300, 450, 600, 900];


let prods = [];
function procesar_datos(tipo_producto, marcas, variedades, cantidades, presentacion) {
    let cant_tot = marcas.length * variedades.length * cantidades.length;
    for (let i = 0; i < cant_tot; i++) {
        let esta=false;
        let codigo;
        let marca;
        let variedad;
        let cantidad;
        do{
            codigo=getCode();
            for (let j=0;j<prods.length;j++){
                if(prods[j].codigos_de_barras.includes(codigo)){
                    esta=true;
                }
            }
        }while(esta)
        
        do{
            marca=elegirAleatoriamente(marcas);
            variedad=elegirAleatoriamente(variedades);
            cantidad=elegirAleatoriamente(cantidades);
            for (let j=0;j<prods.length;j++){
                switch (Object.keys(prods[j].presentacion)){
                    case 'Gr':
                        if(marca==prods[j].marca&&variedad==prods[j].variedad&&cantidad==prods[j].presentacion.Gr==cantidad){
                            esta=true;
                        };
                        break;
                    case 'Un':
                        if(marca==prods[j].marca&&variedad==prods[j].variedad&&cantidad==prods[j].presentacion.Un==cantidad){
                            esta=true;
                        };
                        break;
                    case 'Lt':
                        if(marca==prods[j].marca&&variedad==prods[j].variedad&&cantidad==prods[j].presentacion.Lt==cantidad){
                            esta=true;
                        };
                        break;
                    case 'Ml':
                        if(marca==prods[j].marca&&variedad==prods[j].variedad&&cantidad==prods[j].presentacion.Ml==cantidad){
                            esta=true;
                        };
                        break;
                    case 'CC':
                        if(marca==prods[j].marca&&variedad==prods[j].variedad&&cantidad==prods[j].presentacion.CC==cantidad){
                            esta=true;
                        };
                        break;
                    case 'Kg':
                        if(marca==prods[j].marca&&variedad==prods[j].variedad&&cantidad==prods[j].presentacion.Kg==cantidad){
                            esta=true;
                        };
                        break;                        
                }
                
            }
        }while(esta)

        let costo = getCostPr();
        let codigos = [];
        for (let j = 0; j < elegirAleatoriamente(cantCodes); j++) {
            codigos.push(codigo);
        }
        let prod_act = new Producto(posact, codigos, getSalePr(costo), costo, tipo_producto, marca, variedad, presentacion, cantidad);
        for (let j=0;j<prods.length;j++){
            if (prods[j].marca==prod_act.marca&&prod_act.presentacion&&prods[j].presentacion&&prod_act.variedad==prods[j].variedad){
                
            }        
        }
        prods.push(prod_act)
    }
}
procesar_datos('Chocolate', marcas_chocolates, variedades_chocolate, cantidades_chocolates, 'Gr');
procesar_datos('Arroz',marcas_arroz,variedades_arroz,cantidades_arroz,'Gr');
procesar_datos('Galletitas',marcas_galletitas,variedades_galletitas,cantidades_galletitas,'Gr');

console.log(prods);


let algo3 = {
    "id": 1462,
    "codigos_de_barras": [
        8814336057220,
        66916511474765
    ],
    "precio_de_venta": 7355.470537435858,
    "porcentaje": 7410244.091866396,
    "precio_de_costo": 0.0992595005879583,
    "tipo_producto": "Chocolate",
    "marca": "Godiva",
    "variedad": "Blanco",
    "presentacion": {
        "Gr": 250
    }
}