# [A 3D Skeletonization Algorithm for 3D Mesh Models Using a Partial Parallel 3D Thinning Algorithm and 3D Skeleton Correcting Algorithm](https://www.mdpi.com/2076-3417/7/2/139)

There are three major skeletonization methodologies used in the literature
- distance transform field-based methods
  - extract a skeleton with missing branches
- voronoi diagram methods
  - extract a great number of spurious skeletal branches that are not essential for compact representation of the model
  - time consuming process
- thinning-based methods
  - faster than the Voronoi
  - more detailed skeletons than do those extracted by the field-based methods

However, the existing algorithms cannot preserve the connectivity of the skeletons of the 3D mesh models. The proposed algorithm can extract the skeleton of each branch of a model and preserve the connectivity.
> this is not an issue for crocheteer
>

Various methods of extracting 3D model features have been proposed by many researchers [5,6,7,8,9,10,11,12]

3D skeleton-based approaches have attracted much attention [5,6,7,8,9,10]

There are three major skeletonization methodologies used in the literature:
- distance transform field-based methods [13,14,15],
- Voronoi diagram-based methods [16,17,18],
- and thinning-based methods [19,20,21,22,23]

# [Skeleton Extraction from 3D Point Clouds by Decomposing the Object into Parts](https://arxiv.org/pdf/1912.11932.pdf)
The reader may refer to [15] or [16] for a comprehensive
review of the various skeleton extraction methods that are
available.

# L1-Medial Skeleton of Point Cloud
> seems similar to my solution
>
> even uses a similar weight function
> $w(r) = e^{-r/(h/2)^2}
>> their function looks better
>
> "is a fast decaying smooth function with support radius h defining the size of the supporting local neighborhood for L1-medial skeleton construction"
>
> they take a sample out of a raw scan (downsampling)
> I could probably also skip every X point for efficiency
>
> neighborhood size gradually increases as the algorithm works, this could also be useful especially if a centroid is not stressed
>>

mentioned ROSA algo https://github.com/taiya/rosa