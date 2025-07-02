@echo off
echo Construyendo aplicacion para produccion...
dx build --release

echo Moviendo archivos desde docs/public a docs...
if exist "docs\public" (
    xcopy /s /e /y "docs\public\*" "docs\" > nul
    if %ERRORLEVEL% EQU 0 (
        rd /s /q "docs\public"
        echo Archivos movidos correctamente.
    ) else (
        echo Error al mover los archivos.
        exit /b 1
    )
) else (
    echo La carpeta docs\public no existe. Continuando...
)

echo Configurando routing para SPA (Single Page Application)...
copy "docs\index.html" "docs\404.html"

echo Desplegando a GitHub...
git add .
git commit -m "Deploy to GitHub Pages - %date% %time%"
git push origin main

echo Despliegue completado!
echo Tu app estara disponible en: https://mitdev.cat/
pause
