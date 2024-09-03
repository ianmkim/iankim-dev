Reviving AI Through Heidegger
What contemporary AI can learn from 19th century existential philosophy
Ian M Kim
2023
0.5.1
---
Note* this article was written before the rise of transformers. My thoughts since
have changed considerably.

## Introduction
When the term Artificial Intelligence was coined at the Dartmouth Workshop in
1956, the organizers defined AI beyond narrow automata theory or simply solving
problems more generally; the explicit intention was to re-create human
cognition (McCarthy, 1). Unfortunately, we have considerably strayed from those
goals. Today's thinking machines operate well under narrow contexts but fail to
accomplish elementary human tasks. Moreover, even in those areas where they
work well, they require tremendous resources, and the pace of progress is
grinding to a halt (Dellinger, 2). Something has gone wrong.

So how do we reorient ourselves back to the original definition of artificial
intelligence? To replicate human action, we must first understand the
underlying human experience. One might be tempted to resort to neuroscience or
psychology for answers, but those fields are a red herring. Heidegger instead
gives us a conceptual framework to understand this human experience. If you
gave the best engineer of the 19th century the parts to a Ford Model T, he
would fail spectacularly at building a car. If you additionally told him what
it means to drive a car, he might have a fighting chance. Neuroscience and
psychology merely provide the parts; without understanding first what it means
to be human, we are lost in our efforts (Jonas).

Ultimately, what we want is intelligence that can navigate, act, and interact
in a peopled world: an entity that takes care of our elderly, drives our cars,
builds our buildings, and keeps us company. Only entities that experience the
world as we do can perform these acts. The field desperately needs a renewed
Heidegerrian analysis to reorient us towards achieving artificial general
intelligence.


## Deep Learning
The basis of modern AI is the neural network. A collection of neurons (layer)
is stacked such that every neuron connects to neurons in the next layer. Inputs
go in as numbers on one end, and some useful output comes out on the other.
When numbers go from one layer to the next, these numbers are multiplied by
"weights," which are assigned to connections between neurons. The values of
these weights primarily determine the outcome of the network and its
performance (Rosenblatt, 390). We can determine the optimal combination of
these weights by performing backpropagation (see Rumelhart).

## The Cartesian Assumption
If a whole class of algorithms fails at the most basic human tasks, perhaps we
should reexamine their underlying assumptions. Descartes began with the idea of
an "ego" because he assumed that it is the only thing that can be known for
sure ("cogito ergo sum"). The establishment of this subject necessitates the
existence of an external world from which it can learn (Çüçen 1). With the
subject and object established, epistemology is reformulated as such: the inner
subject knows about a particular object through sense experience, then it
attempts to find out about this object by observing and drawing conclusions
from what is out there (Descartes, Meditation II).

The Cartesian project placed theoretical knowledge above the practical. Because
the object and the subject are detached, knowledge itself becomes detached from
the objective world. Let us take an example of a hammer. Under the traditional
view, the mind becomes aware of itself; then, through our senses and
experiences, we become aware of the hammer. When one recognizes the existence
of a hammer and comes to possess theoretical knowledge about it, they can
perform actions with that knowledge, thereby creating practical knowledge. One
can build theoretical knowledge through deduction, observation, or perhaps an
internal model about how the world operates. The critical point of the
traditional view is that as a result of the subject's isolation from the world,
in order to derive practical knowledge, the subject must first derive
theoretical knowledge through internal means (Heidegger, 86).

Modern AI presupposes the correctness of Descartes. The model is fed a set of
observations about the world from which it tries to derive theoretical
knowledge through backpropagation. The neural network essentially peers out
from its subjective sphere into the world and attempts to learn about it. The
distinction between the subject and the object is painfully apparent. What then
makes up the subjective sphere of a neural network? As a byproduct of
backpropagation, the network gains a certain understanding of the world
embedded in the weights mentioned above. These weights produce a "latent space"
of the world which includes essential information about its observations in
lower-dimensional space. For example, an image 480 pixels wide, 480 pixels high
with three colors (700,000 dimensions), would be accurately represented by just
1000 dimensions when fed through a network. (Simonyan, 3).

We can draw one-to-one parallels from the Cartesian model to our neural
network. Just as man is made aware of a hammer through sense-experience, the
model becomes aware of objects because we intentionally feed it images of
certain objects. While these networks can learn remarkably well what clumps of
pixels look like which objects, if they see a hammer, it will simply show up as
a collection of pixels, not as a hammer in any meaningful way.

When Ren and Lin performed a comprehensive test of a similarly structured
model's ability to generate logically coherent sentences, they found that these
networks perform surprisingly poorly compared to humans (Lin, 8). For example,
when the network was tasked with generating a sentence when given a set of
words like "dog, frisbee, throw, catch," it returned grammatically correct but
logically incoherent sentences like "Two dogs are throwing frisbees at each
other" (Dawson).  While the model may understand what a "dog" looks like, it
certainly cannot understand what a "dog" is in any meaningful way.

The AI community has previously grappled with this issue. In an attempt to
imbue meaning to these clumps of pixels or sets of words, MIT researchers tried
manually enumerating the use-cases for each object in the 60s (Steed, 2). More
recently, networks have been stacked on top of each other in order for the
second network to imbue meaning to the first network's outputs. Unsurprisingly
to any Heideggerian, the Cartesian AI enterprise has been utterly unsuccessful.

## The Heideggerian Alternative
If we want to make general intelligence that lives in the same world as we do,
we better start with what it means to be human and what it means to live in a
human world. Whereas the thinking subject was the origin of Descartes' inquiry,
Heidegger begins with Dasein. Dasein, literally translated as "being there" in
German, is a way of existing in the world: a mode of existence that (so far)
only humans are known to experience. Dasein is characterized by our
being-in-the-world, being-with, and The They.

## Being in the World
Let us reexamine the hammer from a Heideggerian perspective. When we see a
hammer, we do not see an abstract object of lines and colors. The reason that
hammers show up to us as hammers is because Dasein is always concernfully
engaged with the world. Dasein is involuntarily thrown into the world with its
emergent projects and is forced to cope with its environment. Knowledge comes
not as theoretical achievements but rather as a result of practical engagement
(Heidegger, 95):

> "always having to do something, producing something, attending something, making use of something, undertaking… All these ways of Being-in have concerns as their kind of being" (Heidegger, 83).


Hammers show up as meaningful to us in our attempt to accomplish these goals.
If we were inert beings with no engagement with the world, what good would a
hammer ever be?


Hammers also only make sense in a specific equipmental context. What use are
hammers when there are no nails to nail or houses to build? We use the hammer
in order to nail a nail in order to build a house for ourselves. A hammer is
simply incomprehensible in a world without these things. A hammer only exists
as a hammer when the "totality of equipment" exists (Heidegger, 97).

Nevertheless, we hold some theoretical knowledge about hammers; we do not
simply use hammers out of pure habit. Opposite to the Cartesian model,
Heidegger argues that theoretical knowledge, the spatiality or the physics of
the hammer, for instance, is entirely abstracted from the ready-to-hand of
practical knowledge. As a result, we usually only ever view hammers as mere
objects when we encounter cases of breakdown (conspicuousness, obtrusiveness,
obstinacy). If the head of the hammer came off, or there are no more nails, or
the hammer is locked inside the toolbox, only then will you see it as a mere
object or present-at-hand (Heidegger, 102-103). Theoretical knowledge from
backpropagation should therefore not be the primary mode of acquiring
knowledge.


Herein lies the problem for modern AI. Networks exist in isolation and are
spoonfed observations about the world. For example, the YOLOv4 network can
detect objects in an image with remarkable accuracy and speed but only insofar
as it is concerned with present-at-hand knowledge (Bochkovskiy). Hence we see
why our attempts to imbue meaning into the outputs of these programs through
manual enumeration of use-cases or network stacking were hopeless. Genuine
Dasein works the other way around, deriving present-at-hand knowledge from the
ready-to-hand.


So how can we possibly simulate Dasein in artificial models? Researchers from
OpenAI created a physics simulation in which networks were placed and given
explicit goals such as "kick the ball into the soccer net" and "prevent the
ball from going into the net" (Bansal, 4). After millions of simulations, the
agents learned subtle behaviors like tacking, ducking, faking, and kicking the
ball. Training techniques in which agents are placed in a virtual environment
and given a task are referred to as Reinforcement Learning (RL). RL is
essentially a simulation of proto-Dasein: an entity involuntarily thrown into a
world with goals for itself is forced to cope with objects in the world
skillfully: in this case, a ball. For all intents and purposes, the ball is
meaningful to these agents. That ball is something to be shot into the goal or
blocked from going into the goal. These agents do not possess theoretical
present-at-hand knowledge about the ball but rather the practical ready-to-hand
knowledge which allows them to use the ball in order to score goals. The
resurgence and success of RL is a glorious vindication of Heidegger's
formulation of Dasein (Silver). However, RL has limitations also. Applying
learned models to the real world has primarily failed in sophisticated tasks.

## Das Man

While agents learning soccer by themselves is undoubtedly interesting, our
world is more than simply two people with one ball and a soccer goal (if only
things were that easy). Our world is filled with complex objects with complex
meaning that require something more than just concernful engagement. Even in
the simple example, we implicitly insert information about the ball and the
goal into the agent's utility function from our own Dasein. Even though the
agents could physically hug the ball instead of putting it through the goal,
that is simply not what one does with a soccer ball. It comes to us so
naturally that one plays soccer with a soccer ball or hammers with a hammer or
eats with a fork, but what could be the source of this calling?

Heidegger responds with das Man (The They): a normative principle that stems
from the average Dasein (Heidegger, 164). Das Man does not, however, simply
correspond to the statistical average of everyday actions. Heidegger is instead
describing our tendency to conform to expected behavior (Dreyfus, 153). If
there is no normative principle to which we conform, there can be no
equipmental context that creates meaning for objects:


> "If some ate with forks, others with chopsticks and still others used their right hands… etc. whole equipmental nexus involved in cooking and eating a meal could not exist." (Dreyfus, 154).

The categories and distinctions established by das Man can only be experienced
through concernful engagement with the world. For example, it is a lost cause
to try to derive the distinctions of objects through science or philosophy
(theoretical knowledge) simply because das Man defines these categories. It
would be like trying to build a log cabin in a universe where trees do not
exist. For any hammer to show up as a hammer, how one hammers, when one
hammers, and what one hammers must be predetermined.


This predetermination shows up to us in the form of different characteristics
of ready-to-hand objects. We can access, for example, that a dining chair is
meant to be used with a table (equipmental context), that it is made out of a
certain material (whereof), that it is meant for adults with large bodies
(for-whom), for the purposes of eating (towards-which), which we achieve
through sitting on it (use practices) (Heidegger, 97-100). These are
reflections of das Man that only certain embodied beings can access. How can
the use practices of a chair imbued by das Man be intelligible to four-legged
beings? Of course, victims of minor injuries and disabilities remain Dasein
insofar as the essential characteristics of Dasein (such as being-in-the-world)
are physically possible. However, what happens as one becomes more and more
detached from the physical form of the human body?


As we gradually hypothesize a body farther away from the human form, the
characteristics of the ready-to-hand become more unavailable. Ultimately when
you strip away all embodiment, we are left with an entity that no longer finds
objects in the world humanly meaningful because it cannot access the
characteristics of the ready-to-hand and, therefore, das Man.


For instance, door knobs would no longer show up as doorknobs if you had no
hands. The categories set up by das Man would seem less and less intelligible
to you. Let us push this logic to its conclusion: if we perfectly cloned a
brain into a computer, no object in the world would show up as ready-to-hand.
The meaning provided by das Man about how one eats, goes to work, and functions
in society would be utterly meaningless. This disembodied brain would no longer
be considered genuine Dasein. If only genuine Dasein could reason about the
world as we do, we would find that no such system could abstractly reason like
a human on its own.


Thus we arrive at an interesting conclusion that AI, no matter what constitutes
its inner workings, even if it were a perfect copy of the human brain, would
cease to be Dasein if it were not embodied. Without embodiment, there is no
being-with, no das Man, no self-awareness, and therefore no common sense
cognitive abilities in the Heideggerian sense of skillful comportment. This
inevitable conclusion precludes popular conceptions of AGI like HAL-9000 from
2001: A Space Odyssey and most expert portrayals of AGI (see Hutter). Of
course, we could still have ‘lesser Dasein’ which do not perceive the world as
humans do, but if we truly wanted AGI that could exist natively in our world,
the development of bipedal robotics, haptic sensors, and precise control
mechanisms would be a prerequisite to the development of the brain itself.

Even so, embodied concernful engagement alone does not necessarily capture the
whole truth about our ability to gain knowledge about objects. An embodied
network in the wild or RL simulations still uses its weights to represent its
world internally. Heidegger foresaw this mishap and warned against it. If the
world, Daseins, and objects within it gain meaning from each other
interconnectedly, how can networks ever hope to keep an accurate inner
representation of the "whole equipmental nexus"? Tea kettles only make sense
because tea leaves, water, stoves, cups, meals, and customs exist. Even if one
of those factors goes missing, the meaning behind the kettle becomes
irrecoverable. Dreyfus took note of this issue and claimed that human beings
avoid this problem "because their model of the world is the world itself"
(Dreyfus, 1140). Indeed, there is no better representation of the world than
the world itself.

But how can we possibly have an agent that does not keep an internal representation? Heidegger gives us a hint by describing how humans operate:

> "When one is wholly devoted to something and 'really' busies oneself with it, one does not do so just alongside the work itself, or alongside the tool, or alongside both of them 'together' (Heidegger, 405)


Instead, we do not notice the ready-to-handness of the tool at all. The
readiness-to-hand must "withdraw" in order to be "ready-to-hand quite
authentically" (Heidegger, 99). Now it seems like our AI should hold minimal
representations of the world in its mind (as it inevitably must hold some
internal representation). However, the agents should learn to skillfully cope
while mostly referring to sensory inputs. This system would result in an agent
    that learns how to learn to live, not an agent that learns to live: it must
    learn how to extract meaning from das Man, the equipmental context, and its
    surroundings rather than embedding these things internally. For example, a
    traditional network would encode information about the world internally to
    use objects (this is tantamount to humans recognizing objects as
    ready-to-hand while using them, which Heidegger would detest). Instead,
    this new network would act mainly on the input data without utilizing
    internal representations.

## Path Forward

Thus the path forward for AI is cleared by Heidegger. But by uncovering the
path, Heidegger also closed what we thought were shortcuts. If artificial
general intelligence with human capabilities is only possible when it is
concernfully engaged and embodied, we are essentially required to create a copy
of humans. Of course, AGI of 'lesser Dasein' could still be constructed and
used for various purposes (as they are now). Nevertheless, AGI that can
natively exist in our peopled world would require concernful engagement, lack
of reliance on internal representations, and a human-like body to interpret the
meaning granted by das Man.

On the other hand, Heidegger revealed that specific approaches are bound to
fail. For example, methods that assume a solitary subject and a detached world
are bound to fail at but the most meaningless computational tasks (such as
classification or text generation). Therefore, the field should abandon the
Cartesian assumptions that have previously failed them and push onwards,
looking for ways to incorporate parts of Heideggerian phenomenology into
concrete actionable architectures. To the best of our knowledge, this new
architecture will heavily involve reinforcement learning as a training
paradigm, minimize internal representations as much as possible, and be
embodied. Only then will we take our first step on this new path.


## References
 * Bansal, Trapit, Jakub Pachocki, Szymon Sidor, Ilya Sutskever, and Igor Mordatch. "Emergent complexity via multi-agent competition." arXiv preprint arXiv:1710.03748 (2017).
 * Bochkovskiy, Alexey, Chien-Yao Wang, and Hong-Yuan Mark Liao. "Yolov4: Optimal speed and accuracy of object detection." arXiv preprint arXiv:2004.10934 (2020).
 * Cobbe, Karl, Vineet Kosaraju, Mohammad Bavarian, Jacob Hilton, Reiichiro Nakano, Christopher Hesse, and John Schulman. "Training verifiers to solve math word problems." arXiv preprint arXiv:2110.14168 (2021).
 * Dawson, Caitlin. “New Test Reveals AI Still Lacks Common Sense - USC Viterbi: School of Engineering.” USC Viterbi School of Engineering. The University of Southern California, April 9, 2021. https://viterbischool.usc.edu/news/2020/11/new-test-reveals-ai-still-lacks-common-sense/
 * Dellinger, AJ. “The Robot Apocalypse Has Been Delayed until Further Notice.” Mic. Mic, December 4, 2019. https://www.mic.com/impact/artificial-intelligence-development-is-starting-to-slow-down-facebook-head-of-ai-says-19424331
 * Descartes, René. Discourse on method and meditations on first philosophy. Hackett Publishing, 1999.
 * Dreyfus, Hubert L. "Why Heideggerian AI failed and how fixing it would require making it more Heideggerian." Philosophical Psychology 20, no. 2 (2007): 247-268.
 * Hutter, Marcus. "A gentle introduction to the universal algorithmic agent AIXI." Artificial General Intelligence (2003).
 * Jonas, Eric, and Konrad Paul Kording. "Could a neuroscientist understand a microprocessor?." PLoS computational biology 13, no. 1 (2017): e1005268.
 * Lin, Bill Yuchen, Wangchunshu Zhou, Ming Shen, Pei Zhou, Chandra Bhagavatula, Yejin Choi, and Xiang Ren. "CommonGen: A constrained text generation challenge for generative commonsense reasoning." arXiv preprint arXiv:1911.03705 (2019).
 * McCarthy, J., Minsky, M., Rochester, N., Shannon, C.E., A Proposal for the Dartmouth Summer Research Project on Artificial Intelligence., http://raysolomonoff.com/dartmouth/boxa/dart564props.pdf August, 1955
 * Preston, Beth. "Heidegger and artificial intelligence." Philosophy and Phenomenological Research 53, no. 1 (1993): 43-69.
 * Rosenblatt, Frank. "Perceptron simulation experiments." Proceedings of the IRE 48, no. 3 (1960): 301-309.
 * Rosenblatt, Frank. "Perceptron simulation experiments." Proceedings of the IRE 48, no. 3 (1960): 301-309.
 * Silver, David, Aja Huang, Chris J. Maddison, Arthur Guez, Laurent Sifre, George Van Den Driessche, Julian Schrittwieser et al. "Mastering the game of Go with deep neural networks and tree search." nature 529, no. 7587 (2016): 484-489.
 * Simonyan, Karen, and Andrew Zisserman. "Very deep convolutional networks for large-scale image recognition." arXiv preprint arXiv:1409.1556 (2014).
 * Steed, Ryan. "AI is Heideggerian Enough, But Can It Be Authentic?." (2019).
 * Yann LeCun (2016). Slides on Deep Learning Online Archived 2016-04-23 at the Wayback Machine https://indico.cern.ch/event/510372/
 * Çüçen, A. Kadir. "Heidegger’s reading of Descartes’ dualism: The relation of subject and object." In The Paideia Archive: Twentieth World Congress of Philosophy, vol. 6, pp. 57-64. 1998.
