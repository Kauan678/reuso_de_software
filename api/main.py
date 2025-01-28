from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

app = FastAPI()

class Curso(BaseModel):
    titulo: str
    descricao: str
    carga_horaria: int

# banco em memória
banco_cursos = {}


@app.get("/cursos")
def listar_cursos():
    return banco_cursos

@app.get("/cursos/{id}")
def obter_curso(id: int):
    if id not in banco_cursos:
        raise HTTPException(status_code=404, detail="Curso não encontrado")
    return banco_cursos[id]

@app.post("/cursos")
def criar_curso(curso: Curso):
    id = len(banco_cursos) + 1
    banco_cursos[id] = curso
    return {"id": id, "curso": curso}

@app.put("/cursos/{id}")
def atualizar_curso(id: int, curso: Curso):
    if id not in banco_cursos:
        raise HTTPException(status_code=404, detail="Curso não encontrado")
    banco_cursos[id] = curso
    return {"id": id, "curso": curso}

@app.delete("/cursos/{id}")
def excluir_curso(id: int):
    if id not in banco_cursos:
        raise HTTPException(status_code=404, detail="Curso não encontrado")
    del banco_cursos[id]
    return {"mensagem": "Curso excluído com sucesso"}