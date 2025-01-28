import requests

BASE_URL = "http://127.0.0.1:8000"

def listar_cursos():
    response = requests.get(f"{BASE_URL}/cursos")
    print(response.json())

def obter_curso(id):
    response = requests.get(f"{BASE_URL}/cursos/{id}")
    print(response.json())

def criar_curso(titulo, descricao, carga_horaria):
    data = {"titulo": titulo, "descricao": descricao, "carga_horaria": carga_horaria}
    response = requests.post(f"{BASE_URL}/cursos", json=data)
    print(response.json())

def atualizar_curso(id, titulo, descricao, carga_horaria):
    data = {"titulo": titulo, "descricao": descricao, "carga_horaria": carga_horaria}
    response = requests.put(f"{BASE_URL}/cursos/{id}", json=data)
    print(response.json())

def excluir_curso(id):
    response = requests.delete(f"{BASE_URL}/cursos/{id}")
    print(response.json())

if __name__ == "__main__":
    criar_curso("Curso de Python", "Aprenda Python do zero", 40)
    listar_cursos()
    obter_curso(1)
    atualizar_curso(1, "Curso de Python Avançado", "Aprofunde seus conhecimentos", 60)
    listar_cursos()
    excluir_curso(1)
    listar_cursos()

