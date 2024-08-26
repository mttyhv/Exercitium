#input de username:
username = input("username: ")

#coloca o username totalmente em minúsculo:
username = username.lower()

#retira espaços antes e após:
username = username.strip()

#input do primeiro nome:
first_name = input("First name: ")

#input do sobrenome:
last_name = input("Last name: ")

#construção do nome:
full_name = first_name.strip() + " " + last_name.strip()

#exibe o nome da pessoa e indica seu username:
print("Hello, " + full_name.title() + ", you are " + username + "!")