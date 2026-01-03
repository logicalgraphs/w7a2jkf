use book::json_utils::AsJSON;

/*

A parser is either a monad of the form

parse(item, domain) -> (parsed item, domain)

but, as the domain is reduced as parsing occurs, it can also be viewed as a
comonad over the codomain:

codomain parse -> (codomain, parsed item)

So, we can go either way. I understand monads better, but comonads are cooler, 
so ...

*/

// We'll start with the header

/// The w7a header is of the form

/// [Black "Habu Yoshiharu, Oi"]
/// [White "Namekata Hisashi, Challenger"]
/// [Event "54th Oi-sen, Game 1"]
/// [Date "July 10th and 11th 2013"]


parse_header
