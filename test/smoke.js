const { one } = require('./config')

module.exports = scenario => {


// Create and Get an entry
scenario('Can create/get entry', async (s, t) => {
    const { alice, bob } = await s.players({alice: one('alice'), bob: one('bob')}, true)
    
    console.log("Starting happ-example 'smoke' test")

    // Ensure that the Scenerio test Agent IDs are set, and not identical
    t.ok(alice.info('app').agentAddress)
    t.notEqual(alice.info('app').agentAddress, bob.info('app').agentAddress)

    // See if the Agent's Zome whoami API returns real data
    var create = await alice.call( 'app', "example", "create_my_entry", { entry: { content: "Hello, world!" }} );
    console.log("***DEBUG***: create == " + JSON.stringify( create ))
    t.isEquivalent( create.Ok, "QmPipFWaSttS4kroTMYxvW6kKd9rTZcKpPmuM817qGus7c" )

    // Now, get the entry back.  Any agent can get back any entry (they are CAS addresses)
    var get = await alice.call( 'app', "example", "get_my_entry", { address: create.Ok } );
    t.isEquivalent( get.Ok, { App: [ 'my_entry', '{"content":"Hello, world!"}' ] } )
})

}
