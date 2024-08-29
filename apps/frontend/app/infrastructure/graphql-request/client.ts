import { GraphQLClient } from "graphql-request"
import { BACKEND_URI } from "../dotenv";

export const client = new GraphQLClient(BACKEND_URI);


