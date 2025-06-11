function(ctx) {
  // Provide the entire kratos user id to the HTTP request
  user_id: ctx.identity.id
}


// example 
// function(ctx) {
//   userId: ctx.identity.id,
//   customData: ctx.flow.transient_payload.custom_data
//   traits: {
//     email: ctx.identity.traits.email,
//     name: ctx.identity.traits.name,
//     newsletterConsent: ctx.identity.traits.consent.newsletter,
//   },
// }

// function(ctx) {
//     user_id: ctx.identity.id, 
//     traits: { 
//         email: ctx.identity.traits.email
//     }
// }

