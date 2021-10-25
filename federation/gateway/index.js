const { ApolloServer } = require('apollo-server');
const { ApolloGateway } = require("@apollo/gateway");

const gateway = new ApolloGateway({
    serviceList: [
        { name: 'customers', url: 'http://localhost:4001' },
        { name: 'managers', url: 'http://localhost:4002' },
        { name: 'rooms', url: 'http://localhost:4003' }
    ],
});

const server = new ApolloServer({
    gateway,
    subscriptions: false,
});

server.listen().then(({ url }) => {
    console.log(`🚀 Server ready at ${url}`);
});
