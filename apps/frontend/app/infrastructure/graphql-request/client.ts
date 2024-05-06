import { GraphQLClient } from "graphql-request"


const BACKEND_URL = 'http://localhost:8080/graphql';
export const client = new GraphQLClient(BACKEND_URL);


