watch +args='test':
  cargo watch --clear --exec '{{args}}'

ip := '173.255.220.115'

ssh:
  ssh root@{{ip}}

deploy branch=`git branch --show-current`:
  [[ -z `git status --porcelain` ]]
  ssh root@{{ip}} '\
    export DEBIAN_FRONTEND=noninteractive \
    && mkdir -p deploy \
    && apt-get update --yes \
    && apt-get upgrade --yes \
    && apt-get install --yes git'
  ssh root@{{ip}} '[[ -d user.git ]] || git clone --bare https://github.com/casey/user.git'
  git push root@{{ip}}:user.git
  ssh root@{{ip}} '[[ -d deploy ]] || git clone user.git deploy'
  ssh root@{{ip}} 'cd deploy && git reset --hard && git clean -fd && git switch {{branch}} && git pull'
  ssh root@{{ip}} 'cd ./deploy && OPENAI_API_KEY={{env('OPENAI_API_KEY')}} ./setup'
