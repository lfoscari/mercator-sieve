Mercator sieve

The sieve is formed by an array S stored in central memory, which contains URL
signatures. The array is initially empty, and is filled incrementally. The size
of the vector is fixed at some value 𝑛ìn. In mass storage, instead, we will keep
a file Z containing all signatures of previously known URLs in sorted order,
and an auxiliary file A, both initially empty.

Each time a URL u is added to the sieve, we add h(u) to S and u to A. The key
point is what happens when S contains n signatures: in this case, we perform a
flush as follows:

1. We sort S indirectly. That is, we sort stably a vector V of length that
contains the numbers in [0..n) using as key S[i]. At this point V[i]
contains the indeed in S of the signature of rank i (i.e., that i-th signature
in sorted order), and so the signatures S[V[i]] appear in order as i grows.

2. Using this property, we deduplicate S: that is, we mark as useless all
duplicate signatures, using only the first one. Note that we are exploiting the
fact that the sorting algorithm is stable (or we might mark as representative
an occurrence which is not the first one).

3. Now we merge Z with the marked signatures into a new file Z'. We can do
this in linear time and scanning Z sequentially because the S[V[i]]'s are
sorted. We mark the signatures in S which are not duplicates and do not appear
in Z.

4. Finally, we scan A and S in parallel, and for each signature marked in S we
output the corresponding URL in A. Note that the accesses to A is purely
sequential, since S is in the same order (and this is the reason why we
performed an indirect sorting).

5. S and A are emptied, and Z replaced with Z'.

Note that Z at the end of a flush contains again the signatures of all URLs we
have ever seen. Moreover, in output we have produced all and only URLs whose
signature is not part of Z, and thus (modulo collisions) all and only unknown
URLs. Finally, the URL in output are emitted exactly in the order in which they
appear in A for the first time.

credits Sebastiano Vigna @ Unimi
