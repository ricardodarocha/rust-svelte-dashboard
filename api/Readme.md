A API irá conectar no banco de dados Postgre e retornar as últimas n medições na tabela QUIUMETRICAS

```SQL
SELECT * FROM QUIUMETRICAS ORDER BY 1 DESC LIMIT :QTD
```

design da requisição
```Rust
@GET "/api/metrica/{NAME}/{QTD}"
```

Exemplo, pegar as últimas 30 medições do indicador ALFA
```Rust
@GET "/api/metrica/ALFA/30"
```

Irá retornar os seguintes dados

```MD
152|ALFA|31111.00|31072.00| 39.00|
153|ALFA|31114.00|31075.00| 39.00|
154|ALFA|31117.00|31078.00| 39.00|
155|ALFA|31120.00|31081.00| 39.00|
...
182|ALFA|31153.00|31114.00| 39.00|
```

Esta api deverá ser consumida pela aplicação app/dash.exe que irá exibir os dados no formato de gráfico
