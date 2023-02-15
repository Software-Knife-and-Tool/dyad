#!/usr/bin/env bash
BASE=${DYAD_HOME:=/opt/dyad}
BASE_CORE=$BASE/dyad/core

usage () {
    echo "usage: $0 [runtime options] [session options] src-file..." >&2
    echo "[runtime options]" >&2
    echo "  --environment=config-list  environment configuration." >&2
    echo "  --image=image-file         load image-file." >&2
    echo "  --load=src-file            load src-file in sequence." >&2
    echo >&2
    echo "[session options]" >&2
    echo "  -h, --help                 print this message and exit." >&2
    echo "  --script=script-file       run script file and exit." >&2
    echo "  --version                  print version and exit." >&2
    echo "  --eval=form                evaluate form and print result." >&2
    echo "  --quiet-eval=form          evaluate form quietly." >&2
    exit 2
}

SCRIPT=""
ENVIRONMENT=""
IMAGE=""
EVAL=""

CORE=(\
	$BASE_CORE/core.l	     	\
	$BASE_CORE/closures.l  	        \
	$BASE_CORE/fixnums.l            \
	$BASE_CORE/read-macro.l 	\
	$BASE_CORE/read.l       	\
	$BASE_CORE/sequences.l  	\
	$BASE_CORE/symbol-macro.l	\
	$BASE_CORE/symbols.l    	\
	$BASE_CORE/vectors.l            \
        $BASE_CORE/compile.l    	\
        $BASE_CORE/exceptions.l 	\
        $BASE_CORE/format.l     	\
        $BASE_CORE/lambda.l     	\
        $BASE_CORE/lists.l      	\
        $BASE_CORE/load.l       	\
        $BASE_CORE/macro.l      	\
        $BASE_CORE/quasiquote.l 	\
        $BASE_CORE/streams.l    	\
        $BASE_CORE/strings.l    	\
        )

optspec=":h-:"
while getopts "$optspec" optchar; do
    case "${optchar}" in
        -)
            case "${OPTARG}" in
                # loglevel)
                #    val="${!OPTIND}"; OPTIND=$(( $OPTIND + 1 ))
                #    echo "Parsing option: '--${OPTARG}', value: '${val}'" >&2;
                #    ;;
                #loglevel=*)
                #    val=${OPTARG#*=}
                #    opt=${OPTARG%=$val}
                #    echo "Parsing option: '--${opt}', value: '${val}'" >&2
                #    ;;
                eval=*)
                    val=${OPTARG#*=}
                    opt=${OPTARG%=$val}
                    EVAL+="-e ${val} "
                    ;;
                quiet-eval=*)
                    val=${OPTARG#*=}
                    opt=${OPTARG%=$val}
                    EVAL+="-q ${val} "
                    ;;
                script=*)
                    val=${OPTARG#*=}
                    opt=${OPTARG%=$val}
                    SCRIPT="${val}"
                    ;;
                environment=*)
                    val=${OPTARG#*=}
                    opt=${OPTARG%=$val}
                    ENVIRONMENT="-E ${val}"
                    ;;
                image=*)
                    val=${OPTARG#*=}
                    opt=${OPTARG%=$val}
                    IMAGE="-i ${val}"
                    ;;
                help)
                    usage
                    ;;
                version)
                    $BASE/bin/runtime -v
                    exit 2
                    ;;
                *)
                    if [ "$OPTERR" = 1 ] && [ "${optspec:0:1}" != ":" ]; then
                        echo "Unknown option --${OPTARG}" >&2
                    fi
                    ;;
            esac;;
        h)
            usage
            ;;
        *)
            if [ "$OPTERR" != 1 ] || [ "${optspec:0:1}" = ":" ]; then
                echo "Non-option argument: '-${OPTARG}'" >&2
            fi
            ;;
    esac
done

len="${#@}"

SOURCES=""
for (( i=${OPTIND}; i<="${#@}"; i++ )); do SOURCES+=" \"${!i}\"" ; done

export DYAD_LOAD_LIST=SOURCES
$BASE/bin/runtime -d ${ENVIRONMENT} ${CORE[@]} $BASE/dyad/dyad.l ${SOURCES[@]}
