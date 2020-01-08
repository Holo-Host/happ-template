const { one } = require('./config')

module.exports = scenario => {

// Create and Get an entry
scenario('Can create/get entry', async (s, t) => {
    const { alice } = await s.players({alice: one('alice')}, true)
    
    console.log("Starting happ-example 'smoke' test")
    
    var create = await alice.call( 'app', "example", "create_my_entry", { entry: { content: "Hello, world!" }} );
    console.log("***DEBUG***: create == " + JSON.stringify( create ))
    t.isEquivalent( create.Ok, "QmPipFWaSttS4kroTMYxvW6kKd9rTZcKpPmuM817qGus7c" )

    // Now, get the entry back.  Any agent can get back any entry (they are CAS addresses)
    var get = await alice.call( 'app', "example", "get_my_entry", { address: create.Ok } );
    t.isEquivalent( get.Ok, { App: [ 'my_entry', '{"content":"Hello, world!"}' ] } )
})


scenario('Can identify identity and HDK environment', async (s, t) => {
    const { alice, bob } = await s.players({alice: one('alice'), bob: one('bob')}, true)

    // Ensure that the Scenerio test Agent IDs are set, and not identical
    t.ok(alice.info('app').agentAddress)
    t.notEqual(alice.info('app').agentAddress, bob.info('app').agentAddress)

    // Ensure that whoami Zome API agrees
    var alice_whoami = await alice.call( 'app', "example", "whoami", {} );
    console.log("***DEBUG***: whoami == " + JSON.stringify( alice_whoami, null, 2 ))
    t.ok(alice_whoami.Ok)

    // See if the Agent's Zome whoami API returns real data
    t.isEqual(alice_whoami.Ok.agent_address, alice.info('app').agentAddress)
    t.isEquivalent(alice_whoami.Ok.dna_name, "Example")
})


scenario('Can send/receive Message', async (s, t) => {
    const { alice, bob } = await s.players({alice: one('alice'), bob: one('bob')}, true)

    // Send a Message containing some arbitrary Ping data, and ensure we get a Pong back.
    const body = JSON.stringify({"key": "value"})
    const message_reply = await alice.call( 'app', "example", "send_message", {
	to: bob.info('app').agentAddress,
	message: { Ping: "hello" },
	timeout: "2500ms",
    })
    console.log("***DEBUG***: message_reply == " + JSON.stringify( message_reply, null, 2 ))
    t.isEquivalent( message_reply.Ok, { Pong: [ alice.info('app').agentAddress, "hello" ] })
})

}
