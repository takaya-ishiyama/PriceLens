import { GraphQLClient } from "graphql-request";
import { BACKEND_URI } from "../dotenv";

export const client = new GraphQLClient(BACKEND_URI); // .envがサーバーサイドでしか使えないから諸々設定する必要がある
// 参考：https://github.com/remix-run/remix/issues/1186
// export const client = new GraphQLClient("http://localhost:8080/graphql");
