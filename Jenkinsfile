pipeline {
	agent any
	stages {
		stage('Lint HTML'){
			sh 'tidy -q -e *.html'
		}
		stage('Upload to AWS') {
			steps {
                	 withAWS(region:'us-west-2',credentials:'aws-static1') {
                 	sh 'echo "Uploading content with AWS creds"'
                     	s3Upload(pathStyleAccessEnabled: true, payloadSigningEnabled: true, file:'index.html', bucket:'evan-udacity-jenkins')
			sh 'echo "Hello World"'
			sh '''
				echo "Multiline shell steps works too"
				ls -lah
			'''
			 }
			}
		}			
	}
}
