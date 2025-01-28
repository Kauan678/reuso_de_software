from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import requests

BASE_URL = "http://127.0.0.1:8000"

app = FastAPI()

@app.get("/cursos")
def listar_cursos():
    return requests.get(f"{BASE_URL}/cursos").json()

@app.get("/cursos/{id}")
def obter_curso(id):
    return requests.get(f"{BASE_URL}/cursos/{id}").json()

@app.post("/cursos")
def criar_curso(titulo, descricao, carga_horaria):
    data = {"titulo": titulo, "descricao": descricao, "carga_horaria": carga_horaria}
    return requests.post(f"{BASE_URL}/cursos", json=data).json()

@app.put("/cursos/{id}")
def atualizar_curso(id, titulo, descricao, carga_horaria):
    data = {"titulo": titulo, "descricao": descricao, "carga_horaria": carga_horaria}
    response = requests.put(f"{BASE_URL}/cursos/{id}", json=data)
    return response.json()

@app.delete("/cursos/{id}")
def excluir_curso(id):
    response = requests.delete(f"{BASE_URL}/cursos/{id}")
    return response.json()