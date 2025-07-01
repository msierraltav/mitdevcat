@echo off
echo Construyendo aplicacion...
dx bundle --out-dir docs

echo Moviendo archivos estaticos...
if exist "docs\public" (
    echo Encontrada carpeta docs\public, moviendo contenido...
    if exist "docs\public\assets" (
        if not exist "docs\assets" mkdir "docs\assets"
        move "docs\public\assets\*" "docs\assets\"
    )
    if exist "docs\public\wasm" (
        if not exist "docs\wasm" mkdir "docs\wasm"
        move "docs\public\wasm\*" "docs\wasm\"
    )
    move "docs\public\*" "docs\" 2>nul
    rmdir "docs\public" 2>nul
    echo Archivos movidos correctamente.
) else (
    echo No se encontro carpeta docs\public, continuando...
)

echo Verificando estructura de archivos...
if exist "docs\assets" (
    echo ✓ Carpeta docs\assets existe
) else (
    echo ✗ ADVERTENCIA: No se encontro carpeta docs\assets
)

echo Configurando routing...
copy "docs\index.html" "docs\404.html"

echo Desplegando a GitHub...
git add .
git commit -m "Deploy to GitHub Pages - %date% %time%"
git push origin main

echo Despliegue completado!
echo Tu app estara disponible en: https://tu-usuario.github.io/mitdevcat/
pause
