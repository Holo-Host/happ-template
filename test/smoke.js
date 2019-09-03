module.exports = scenario => {

// Create and Get an entry
scenario('Can create/get entry', async (s, t, {alice, bob}) => {
    console.log("Starting happ-example 'smoke' test")

    // Ensure that the Scenerio test Agent IDs are set, and not identical
    t.ok(alice.app.agentId)
    t.notEqual(alice.app.agentId, bob.app.agentId)

    // See if the Agent's Zome whoami API returns real data
    var create = await alice.app.call( "example", "create_my_entry", { entry: { content: "Hello, world!" }} );
    console.log("***DEBUG***: whoami == " + JSON.stringify( create ))
    t.isEquivalent( create.Ok, "QmPipFWaSttS4kroTMYxvW6kKd9rTZcKpPmuM817qGus7c" )

    // Now, get the entry back.  Any agent can get back any entry (they are CAS addresses)
    var get = await alice.app.call( "example", "get_my_entry", { address: create.Ok } );
    t.isEquivalent( get.Ok, { App: [ 'my_entry', '{"content":"Hello, world!"}' ] } )
})

}
