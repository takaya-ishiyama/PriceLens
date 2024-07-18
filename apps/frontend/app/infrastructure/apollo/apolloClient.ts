import { ApolloClient, InMemoryCache } from '@apollo/client';

// require('dotenv').config()


// const BACKEND_URL = process.env.BACKEND_URI;
const BACKEND_URL = 'http://localhost:8080/graphql';

const apolloClient = new ApolloClient({
  uri: BACKEND_URL,
  cache: new InMemoryCache(),
});
export default apolloClient
