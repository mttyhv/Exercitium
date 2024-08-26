#lista de convidados:
convidados = ['Jesus Cristo', 'Buda', 'Maomé']

#convite individual:
for convidado in convidados:
	print('Olá ' + convidado + ', gostaria de um jantar?')

#demonstra que um convidado não poderá comparecer:
print('Infelizmente ' + convidados.pop(2) + ' não poderá comparecer.')

#insere novo convidado no fim da lista:
convidados.append('Zaratustra')

#convite individual:
for convidado in convidados:
	print('Olá ' + convidado + ', gostaria de um jantar?')

#adicionar mais convidados:
print('Encontramos uma mesa de jantar maior...')
convidados.insert(0, 'Helena Blavatsky')
convidados.insert(3, 'Saint German')
convidados.append('Carl Jung')

#convite individual:
for convidado in convidados:
	print('Olá ' + convidado + ', gostaria de um jantar?')