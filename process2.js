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

let posact = 0;
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
marcas_fideos= ["Barilla","Ronzoni","De Cecco","San Remo","La Molisana","Buitoni","Colavita","Giovanni Rana","Knorr","Catelli","Panzani",    "Misko",    "Ancient Harvest",    "Dreamfields",    "Mueller's",    "Hodgson Mill",    "Skinner",    "No Yolks",    "Annie Chun's", "Schar",];
variedades_fideos=["Espaguetis","Fettuccine","Linguine","Penne","Rigatoni","Farfalle","Conchiglie","Rotini","Soba","Udon","Ramen","Capellini","Tagliatelle","Lasagna","Macarrones","Fideos de arroz","Somen","Pappardelle","Orzo","Cavatelli",];
marcas_yerbas=["Amanda","La Tranquera","Rosamonte","Playadito","Taragüí","Union","Canarias","Nobleza Gaucha","Santo Pipo","Cruz de Malta","Aguantadora","Pajarito","Kraus","La Merced","Selecta","CBSe","Barão","Piporé","Yerba Mate Campesino","Mate Factor","Nativa","Sara","Simpli Mate","Verde Mate","La Potente","Mate Kurupí","Las Marías","Kurupi","Pipore","Coopetrabas","Del Cebador","Esperanza","Kraus Organic","Cruz de Malta Tereré","Sol de Acuario","Campesino Orgánico","Yacuy","Biofresco","Cachamate","Yerba Pajarito Selva","Mañanita","BIO Yerba Mate","Rosamonte Especial","Cruceña","Kerbal",];
variedades_yerbas=["Clásica", "Suave", "Sin Palo", "Bajo Contenido de Polvo"];
cantidades_yerbas=[250,500,1000];
marcas_mermeladas=["Smucker's","Bonne Maman","Stonewall Kitchen","Welch's","Hero","Sarabeth's","Tiptree","Bama","Polaner","Mrs. Bridges","Wilkin & Sons","Dickinson's","Frog Hollow Farm","Crofters","Bonne Maman","Fruitfield","Duerr's","Hartley's","Beerenberg","Rigoni di Asiago","St. Dalfour","Braswell's","Buderim Ginger","Folláin","Bonne Maman","Welch's","Tiptree","Bon Maman","Mackays","L'arbre Vert","Bar-le-Duc","Bionaturae","Clearspring","Dalfour","D'arbo",  ];
variedades_mermeladas=["Fresa","Frambuesa","Arándano","Ciruela","Mora","Melocotón","Naranja","Mandarina","Piña","Manzana","Cereza","Albaricoque","Higo","Uva","Kiwi","Pomelo","Limón","Jengibre","Coco","Arándano rojo","Mango","Pera","Granada","Melón","Frutas del bosque","Tomate","Papaya","Canela","Vainilla","Rosa","Pimiento","Elderberry","Zanahoria",];
marcas_harinas=["Gold Medal","King Arthur Flour","Bob's Red Mill","Pillsbury","Robin Hood","White Lily","Heckers","Arrowhead Mills","Caputo","Antimo Caputo","Great River Organic Milling","Central Milling","Hodgson Mill","Goya","King Milling","Organic Gemini","Jovial","Namaste Foods","Authentic Foods","Bobs Well Flour","Honeyville","Azure Market Organics","Anna","Martha White","Doves Farm","Red Star","Pamela's Products","Krusteaz","Quaker","Admiral","Bacheldre Mill","Breadtopia","BRM","Ceresota","Community Grains","ConAgra Mills","Cup4Cup","Dietz & Watson",];
variedades_harinas=["Harina 000","Harina 0000","Harina Integral","Harina de Maíz","Harina Leudante","Harina de Garbanzo","Harina de Arroz","Harina de Maíz Precocida","Harina de Soja","Harina de Almendra",];
marcas_panes=["Bimbo","Lactal","Fargo","Bagley","San Cayetano","Grandiet","Calsa","Grupo Los Grobo","Forcela","Vea","Wentz","Faverin","Nutrella","Calsa","2 de Oro","Oki","Vizzolini","Argentinos","Morella","La Tradicional","La Salteña","Lory","Don Satur","Tante María","Viguisa",];
variedades_panes=["Blanco","Integral","de Centeno","Multicereal","de Salvado","de Soja","de Avena","de Espelta","de Maíz","de Semillas","de Nueces","de Pasas","de Aceitunas","de Queso","de Pita","Baguette","Ciabatta","de Molde","de Linaza","de Chía","de Kamut","de Quinoa","Brioche","Challah","Naan","Focaccia","Lavash","de Centeno Integral","sin Gluten","de Calabacín","de Zanahoria","de Tomate Seco","de Remolacha","de Cerveza",];
cantidades_panes=[350,400,500,600];
marcas_gaseosas = ["Coca-Cola","Pepsi","Sprite","Fanta","Dr Pepper","7UP","Mountain Dew","Canada Dry","Schweppes","Crush","Sunkist","A&W Root Beer","Barq's","Mirinda","RC Cola","Big Red","Cheerwine","Squirt","Jarritos","Inca Kola"];
variedades_gaseosas = ["Cola","Lima-limón","Naranja","Uva","Fresa","Manzana","Cereza","Frambuesa","Piña","Jengibre","Toronja","Limón","Tónica","Mango","Melocotón","Cítricos mixtos","Frutas tropicales","Vainilla","Root Beer","Kiwi"];
cantidades_gaseosas_ml = [354, 500];
cantidades_gaseosas_lt = [1.5,2.25,3];

let prods = [];
function procesar_datos(tipo_producto, marcas, variedades, cantidades, presentacion) {
    let cant_tot = marcas.length * variedades.length * cantidades.length;
    for (let i = 0; i < cant_tot; i++) {
        let costo = getCostPr();
        let codigos = [];
        for (let j = 0; j < elegirAleatoriamente(cantCodes); j++) {
            let code;
            let esta=true;
            while (esta){
                esta=false;
                code=getCode();
                for (let h=0;h<prods.length;h++){
                    if (prods[h].codigos_de_barras.includes(code))
                    esta=true;
                }
            }
            codigos.push(getCode());
        }
        let prod_act = new Producto(posact, codigos, getSalePr(costo), costo, tipo_producto, elegirAleatoriamente(marcas), elegirAleatoriamente(variedades), presentacion, elegirAleatoriamente(cantidades));
        let esta=false;
        for (let j=0;j<prods.length;j++){
            if (prod_act.marca==prods[j].marca&&prod_act.variedad==prods[j].variedad&&prod_act.presentacion==prods[j].presentacion){
                esta=true
            }
        }
        if(!esta)
        prods.push(prod_act)
        posact++;
    }
}
procesar_datos('Chocolate', marcas_chocolates, variedades_chocolate, cantidades_chocolates, 'Gr');
procesar_datos('Arroz',marcas_arroz,variedades_arroz,cantidades_arroz,'Gr');
procesar_datos('Galletitas',marcas_galletitas,variedades_galletitas,cantidades_galletitas,'Gr');
procesar_datos('Fideos',marcas_fideos,variedades_fideos,[500],'Gr');
procesar_datos('Yerba', marcas_yerbas,variedades_yerbas,cantidades_yerbas,'Gr');
procesar_datos('Mermelada',marcas_mermeladas,variedades_mermeladas,[400],'Gr');
procesar_datos('Harina',marcas_harinas,variedades_harinas,[1],'Kg');
procesar_datos('Pan De Mesa',marcas_panes,variedades_panes,cantidades_panes,'Gr');
procesar_datos('Gaseosa',marcas_gaseosas,variedades_gaseosas,cantidades_gaseosas_ml,'Ml');
procesar_datos('Gaseosa', marcas_gaseosas, variedades_gaseosas, cantidades_gaseosas_lt, 'Lt');


console.log(JSON.stringify(prods));

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