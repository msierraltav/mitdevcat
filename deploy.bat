@echo off
echo Construyendo aplicacion...
dx bundle --out-dir docs

echo Moviendo archivos estaticos...
if exist "docs\public" (
    move docs\public\* docs\
    rmdir docs\public
)

echo Configurando routing...
copy docs\index.html docs\404.html

echo Desplegando a GitHub...
git add .
git commit -m "Deploy to GitHub Pages - %date% %time%"
git push origin main

echo Despliegue completado!
pause
