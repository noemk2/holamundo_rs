Hola mundo en near con Rust 
==================

Introducción a holamundo en near (Rust)
==================

 un holamundo en near protocol, este contrato te perminte:
 
 1. print "Hello world" 
 2. print "Hello " + $USER
 

👨‍💻 Instalación en local
===========

Para correr este proyecto en local debes seguir los siguientes pasos:

Paso 1: Pre - Requisitos
------------------------------
1. Asegúrese de tener todos los [prequisitos]
3. Instalar dependencias: `cargo check`
4. Crear un test near account [NEAR test account]
5. Instalar el NEAR CLI globally: [near-cli] es una interfaz de linea de comando (CLI) para interacturar con NEAR blockchain

    yarn install --global near-cli

Step 2: Configura tu NEAR CLI
-------------------------------

Configura tu near-cli para autorizar su cuenta de prueba creada recientemente:

    near login
    
Step 3: Clonar Repositorio
-------------------------------    

Este comando nos permite clonar el repositorio de nuestro proyecto 

```bash
git clone https://github.com/noemk2/holamundo_as.git
```

Una vez que hayas descargado el repositorio, asegurate de ejecutar los comandos dentro del repositorio descargado. Puedes hacerlo con
```bash
cd holamundo_as/
```

Step 4: Realiza el BUILD para implementación de desarrollo de contrato inteligente 
------------------------------------------------------------------------------------

Instale el gestor de dependencia de Node.js dentro del repositorio

```bash
npm install
```

Cree el código de contrato inteligente e implemente el servidor de desarrollo local: 
```bash
yarn deploy:dev
```

Cree la variable local $CONTRACT_NAME (permite guardar tu contrato temporal en una variable facil de recordar)
```bash
source ./neardev/dev-account.env
```

Consulte` package.json` para obtener una lista completa de `scripts` que puede ejecutar con` yarn`). Este script le devuelve un contrato inteligente provisional
implementado (guárdelo para
usarlo más tarde)


¡Felicitaciones, ahora tendrá un entorno de desarrollo local ejecutándose en NEAR TestNet!




✏️ Comando  view : request estatico
-----------------------------------------------

Permite imprimir "Hello world" 

Para Linux:
```bash
near view $CONTRACT_NAME hello_world --account-id <username>.testnet
```

✏️ Comando  call : request dinamico
--------------------------------------------

Permite imprimir "Hello " + <username>.testnet  

Para Linux :
```bash
near call $CONTRACT_NAME hello --account-id <username>.testnet
```


🤖 Test 
==================

Las pruebas son parte del desarrollo, luego, para ejecutar las pruebas en el contrato inteligente , debe ejecutar el siguiente comando:

    yarn test


==============================================

  [create-near-app]: https://github.com/near/create-near-app
  [prequisitos]: https://github.com/near/near-sdk-rs#pre-requisites
  [NEAR accounts]: https://docs.near.org/docs/concepts/account
  [NEAR Wallet]: https://wallet.testnet.near.org/
  [near-cli]: https://github.com/near/near-cli
  [NEAR test account]: https://docs.near.org/docs/develop/basics/create-account#creating-a-testnet-account
  [nvm]: https://github.com/nvm-sh/nvm
  [UX/UI]: https://www.figma.com/proto/GqP5EF5zRZRvAv3HoaSsuN/uniwap?node-id=39%3A2300&scaling=min-zoom&page-id=0%3A1&starting-point-node-id=39%3A2300&hide-ui=1
  [UX/UI]: https://www.figma.com/proto/0dZLC0WI1eVsfjeKu3T8J8/Garant%C3%ADzame?node-id=2%3A8&scaling=scale-down-width&page-id=0%3A1&starting-point-node-id=2%3A8
