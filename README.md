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
1. Asegúrese de tener todos los [prequisitos para compilar en rust] (Install Rustup , Add wasm target to your toolchain)
3. Crear un test near account [NEAR test account]
4. Instalar el NEAR CLI globally: [near-cli] es una interfaz de linea de comando (CLI) para interacturar con NEAR blockchain

    yarn install --global near-cli

Step 2: Configura tu NEAR CLI
-------------------------------

Configura tu near-cli para autorizar su cuenta de prueba creada recientemente:

    near login
    
Step 3: Clonar Repositorio
-------------------------------    

Este comando nos permite clonar el repositorio de nuestro proyecto 

```bash
git clone https://github.com/noemk2/holamundo_rs.git
```

Una vez que hayas descargado el repositorio, asegurate de ejecutar los comandos dentro del repositorio descargado. Puedes hacerlo con
```bash
cd holamundo_rs/
```

Step 4: Realiza el BUILD para implementación de desarrollo de contrato inteligente 
------------------------------------------------------------------------------------

Instalar dependencias 

```bash
cargo check
```

Cree el código de contrato inteligente e implemente el servidor de desarrollo local: 
```bash
sh build.sh
```

Cree la variable local $CONTRACT_NAME (permite guardar tu contrato temporal en una variable facil de recordar)
```bash
source ./neardev/dev-account.env
```

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

Permite imprimir "Hello " + <username> .testnet  

Para Linux :
```bash
near call $CONTRACT_NAME hello --account-id <username>.testnet
```


🤖 Test 
==================

Las pruebas son parte del desarrollo, luego, para ejecutar las pruebas en el contrato inteligente , debe ejecutar el siguiente comando:

```bash
 cargo test -- --nocapture  
```


==============================================

  [create-near-app]: https://github.com/near/create-near-app
  [prequisitos para compilar en rust]: https://github.com/near/near-sdk-rs#pre-requisites
  [NEAR accounts]: https://docs.near.org/docs/concepts/account
  [NEAR Wallet]: https://wallet.testnet.near.org/
  [near-cli]: https://github.com/near/near-cli
  [NEAR test account]: https://docs.near.org/docs/develop/basics/create-account#creating-a-testnet-account
