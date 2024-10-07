import { GraphQLClient } from "graphql-request";


export const client = (uri: string) => new GraphQLClient(uri);


