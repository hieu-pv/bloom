<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center">
      <v-col cols="12">
        <h2>Connections</h2>
        <p>
          Connect all your apps to start automating them.
        </p>
      </v-col>

      <v-col cols="12" class="ma-0 py-0">

        <v-app-bar dense color="white" flat>
          <v-spacer />
          <v-btn :to="`/${projectFullPath}/-/bots/connections/new`" color="success" depressed>
            <v-icon left>mdi-plus</v-icon>
            Connect App
          </v-btn>
        </v-app-bar>
      </v-col>


      <v-col cols="12">
        <b-connections-list :loading="loading" :connections="connections" />
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { BotConnection, Project } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import BConnectionsList from '@/ui/components/bots/connections_list.vue';

export default VueApp.extend({
  name: 'BBotsConnectionsView',
  components: {
    BConnectionsList,
  },
  data() {
    return {
      loading: false,
      error: '',
      project: null as Project | null,
    };
  },
  computed: {
    connections(): BotConnection[] {
      return this.project?.botsConnections ?? [];
    },
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.project = await this.$botsService.fetchBotsConnectionsForProject(this.projectFullPath);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
